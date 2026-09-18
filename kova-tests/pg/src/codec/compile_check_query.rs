#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Options;
use kova::dsl::LockMode;
use kova::dsl::func::CountAll;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Stress Test: Custom Types with PostgreSQL-specific boundaries
async fn compile_check_custom_types() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let target_uuid = Uuid::new_v4();
    let current_time = Utc::now();
    let site = Url::parse("https://rust-lang.org")?;

    let dummy_log = EventLog {
        id: target_uuid.clone(),
        description: "event log".into(),
        created_at: current_time.clone(),
        updated_at: Timestamp::now(),
        website: site.clone(),
        metadata: HashMap::from([
            ("status".into(), "active".into()),
            ("source".into(), "api".into())
        ]),
    };

    client.insert(&dummy_log).execute().await?;

    let _upsert_id = client.insert(&dummy_log)
        .on_conflict(EventLog::id)
        .update((
            EventLog::updated_at.set(Timestamp::now()),
            EventLog::metadata.excluded(),
        ))
        .returning(EventLog::id)
        .execute_in::<Uuid>()
        .await?;

    let mut new_meta = HashMap::new();
    new_meta.insert("archived".into(), "true".into());

    // Test for native foreign types in the Filter tree (Eq, NotIn, Gt)
    let _updated_urls = client.update::<EventLog>()
        .set((
            EventLog::metadata.set(new_meta),
            EventLog::website.set(Url::parse("https://crates.io").unwrap()),
            EventLog::updated_at.set(Timestamp::now())
        ))
        .filter(
            EventLog::id.eq(&target_uuid)
                & EventLog::created_at.gt(&current_time)
                & EventLog::metadata.is_not_null()
        )
        .returning(EventLog::website)
        .execute_in::<Url>()
        .await?;

    // 4. Delete + Returning
    let _deleted_time = client.delete::<EventLog>()
        .filter(EventLog::website.eq(&site))
        .returning(EventLog::created_at)
        .execute_in::<DateTime<Utc>>()
        .await?;

    let _logs = client.select::<EventLog>()
        .filter(EventLog::id.eq(&target_uuid))
        .load_as::<EventLog>()
        .await?;

    // Ad-Hoc heterogeneous tuple projection + pessimistic locking
    // (test for bind_type! macros and locking mechanisms)
    let _tuples = client.select::<EventLog>()
        .columns((
            EventLog::id,
            EventLog::website,
            EventLog::created_at,
            EventLog::updated_at,
            CountAll
        ))
        .group_by((EventLog::id, EventLog::website, EventLog::created_at, EventLog::updated_at))
        .order_by(EventLog::created_at.desc())
        .lock_all(LockMode::Update, None)
        .load_in::<(
            Uuid,
            Url,
            DateTime<Utc>,
            Timestamp,
            i64
        )>()
        .await?;

    // Option boundary extraction test
    let _optional_urls = client.select::<EventLog>()
        .columns(EventLog::website)
        .load_in::<Option<Url>>()
        .await?;

    let _extracted_maps = client.select::<EventLog>()
        .columns((EventLog::website, EventLog::metadata))
        .load_in::<(Option<Url>, HashMap<String, String>)>()
        .await?;

    Ok(())
}