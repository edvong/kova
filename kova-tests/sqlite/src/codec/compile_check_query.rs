#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Options;
use kova::dsl::func::{Count, Wildcard};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Stress Test: Custom Types, Reference Penetration, and Custom Compatible Rules on SQLite
fn compile_check_custom_types() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    let target_uuid = Uuid::new_v4();
    let current_time = Utc::now();
    let site = Url::parse("https://rust-lang.org")?;

    let dummy_log = EventLog {
        id: target_uuid,
        description: "event log".into(),
        created_at: current_time,
        updated_at: Timestamp::now(),
        website: site.clone(),
        metadata: HashMap::from([
            ("status".into(), "active".into()),
            ("source".into(), "api".into()),
        ]),
    };
    client.insert(&dummy_log).execute()?;

    // 1. Upsert + Returning bare custom type (Uuid)
    let _upsert_id = client
        .insert(&dummy_log)
        .on_conflict(EventLog::id)
        .update((EventLog::updated_at.set(Timestamp::now()), EventLog::metadata.excluded()))
        .returning(EventLog::id)
        .execute_in::<Uuid>()?;

    let mut new_meta = HashMap::new();
    new_meta.insert("archived".into(), "true".into());

    // 2. Update with borrowed HashMap, custom Compatible (&str -> Url), and Returning bare Url
    let _updated_urls = client
        .update::<EventLog>()
        .set((
            EventLog::metadata.set(&new_meta),
            EventLog::website.set("https://crates.io"), // Custom Compatible<Static<Url>, AppCodec> for Static<&str>
            EventLog::updated_at.set(Timestamp::now()),
        ))
        .filter(
            EventLog::id.eq(&target_uuid)
                & EventLog::created_at.gt(&current_time)
                & EventLog::website.in_list(["https://rust-lang.org", "https://docs.rs"])
                & EventLog::metadata.is_not_null(),
        )
        .returning(EventLog::website)
        .execute_in::<Url>()?;

    // 3. Delete + Returning bare DateTime<Utc>
    let _deleted_time = client
        .delete::<EventLog>()
        .filter(EventLog::website.eq(&site) | EventLog::website.eq("https://old.site"))
        .returning(EventLog::created_at)
        .execute_in::<DateTime<Utc>>()?;

    // 4. Select with owned and borrowed custom types
    let _logs = client
        .select::<EventLog>()
        .filter(
            EventLog::id.eq(target_uuid)
                & EventLog::created_at.between(&current_time, Utc::now()),
        )
        .load_as::<EventLog>()?;

    // 5. Ad-Hoc heterogeneous tuple projection
    let _tuples = client
        .select::<EventLog>()
        .columns((
            EventLog::id,
            EventLog::website,
            EventLog::created_at,
            EventLog::updated_at,
            Count(Wildcard),
        ))
        .group_by((EventLog::id, EventLog::website, EventLog::created_at, EventLog::updated_at))
        .order_by(EventLog::created_at.desc())
        .load_in::<(Uuid, Url, DateTime<Utc>, Timestamp, i64)>()?;

    // 6. Option<CustomType> and Box<CustomType> extraction
    let _optional_urls =
        client.select::<EventLog>().columns(EventLog::website).load_in::<Option<Url>>()?;

    let _boxed_uuids =
        client.select::<EventLog>().columns(EventLog::id).load_in::<Box<Uuid>>()?;

    let _extracted_maps = client
        .select::<EventLog>()
        .columns((EventLog::website, EventLog::metadata))
        .load_in::<(Option<Url>, HashMap<String, String>)>()?;

    Ok(())
}
