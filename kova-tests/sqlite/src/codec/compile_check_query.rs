use std::collections::HashMap;
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use url::Url;
use uuid::Uuid;
use kova::Options;
use kova::dsl::func::CountAll;
use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use crate::Result;

async fn compile_check_custom_types() -> Result<()> {
    let client = Options::from("").build()?.with::<AppCodec>();

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
    client.insert(&dummy_log).execute()?;


    let mut new_meta = HashMap::new();
    new_meta.insert("archived".into(), "true".into());

    // Test for native foreign types in the Filter tree (Eq, NotIn, Gt)
    client.update::<EventLog>()
        .set((
            EventLog::metadata.set(new_meta),
            EventLog::website.set(Url::parse("https://crates.io").unwrap()),
            EventLog::updated_at.set(Timestamp::now())
        ))
        .filter(EventLog::id.eq(&target_uuid))
        .filter(
            EventLog::id.eq(&target_uuid)
                & EventLog::created_at.gt(current_time)
                & EventLog::metadata.is_not_null()
        )
        .execute()?;

    let _logs = client.select::<EventLog>()
        .filter(EventLog::id.eq(target_uuid))
        .load_as::<EventLog>()?;

    // Ad-Hoc Heterogeneous Tuple Projection (Test bind_type! Macro)
    let _tuples = client.select::<EventLog>()
        .columns((
            EventLog::id,
            EventLog::website,
            EventLog::created_at,
            EventLog::updated_at,
            CountAll
        ))
        .group_by((EventLog::id, EventLog::website, EventLog::created_at, EventLog::updated_at))
        .load_in::<(
            Uuid,
            Url,
            DateTime<Utc>,
            Timestamp,
            i64
        )>()?;

    // Option boundary extraction test
    let _optional_urls = client.select::<EventLog>()
        .columns(EventLog::website)
        .load_in::<Option<Url>>()?;

    let _extracted_maps = client.select::<EventLog>()
        .columns((EventLog::website, EventLog::metadata))
        .load_in::<(Option<Url>, HashMap<String, String>)>()?;

    Ok(())
}