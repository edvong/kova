use crate::Result;
use crate::type_check::codec::AppCodec;
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Options;
use kova::dsl::func::{Abs, Avg, Json, JsonEach, JsonTree, Length, Lower, Sum, Upper};
use kova::dsl::func::{Count, Wildcard};
use models::type_check::event_log::{AuditPayload, Decimal, EventLog};
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
            EventLog::id.eq(target_uuid) & EventLog::created_at.between(&current_time, Utc::now()),
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

    let _boxed_uuids = client.select::<EventLog>().columns(EventLog::id).load_in::<Box<Uuid>>()?;

    let _extracted_maps = client
        .select::<EventLog>()
        .columns((EventLog::website, EventLog::metadata))
        .load_in::<(Option<Url>, HashMap<String, String>)>()?;

    Ok(())
}

/// Stress Test 2: Codec-Aware Domain Capabilities (`CanMath<X>`, `CanString<X>`, `CanSqliteJson<X>`)
fn compile_check_custom_domain_capabilities() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    let base_fee = Decimal(100);
    let _audit = AuditPayload { actor: "system".into(), level: 3 };

    // 1. CanMath<AppCodec> on `Column<Decimal>` and `Column<Option<Decimal>>`
    let _math_rows = client
        .select::<EventLog>()
        .columns((
            EventLog::cost.add(&base_fee).sub(10_i64).mul(2_i64).div(Decimal(5)),
            EventLog::opt_cost.add(50_i64).sub(&base_fee),
            Sum(EventLog::cost),
            Avg(EventLog::opt_cost),
            Abs(EventLog::cost.sub(1000_i64)),
        ))
        .filter(
            EventLog::cost.add(100_i64).gt(Decimal(500))
                & EventLog::opt_cost.mul(2_i64).le(1000_i64),
        )
        .group_by((EventLog::cost, EventLog::opt_cost))
        .having(Sum(EventLog::cost).gt(0_i64) & Avg(EventLog::opt_cost).le(&base_fee))
        .load_in::<(Decimal, Option<Decimal>, Decimal, Option<Decimal>, Decimal)>()?;

    client
        .update::<EventLog>()
        .set((
            EventLog::cost.increment(50_i64),
            EventLog::cost.decrement(&base_fee),
            EventLog::opt_cost.increment(Decimal(10)),
        ))
        .execute()?;

    // 2. CanString<AppCodec> (automatically unlocking CanSqliteString & CanSqliteFts) on `Url` / `Option<Url>`
    let _str_and_fts_rows = client
        .select::<EventLog>()
        .columns((
            Lower(EventLog::website),
            Upper(EventLog::opt_website),
            Length(EventLog::website),
            EventLog::website.bm25().weights([1.0, 0.5]),
            EventLog::website.highlight(0, "<b>", "</b>"),
            EventLog::opt_website.snippet(0, "[", "]", "...", 16),
        ))
        .filter(
            EventLog::website.like("https://%")
                & EventLog::opt_website.not_like("http://%")
                & EventLog::website.regexp("^https://[a-z]+")
                & EventLog::opt_website.regexp("sqlite\\.org$")
                & EventLog::website.matches("rust AND sqlite")
                & Length(EventLog::website).gt(10_i64),
        )
        .order_by(EventLog::website.bm25().asc())
        .load_in::<(String, String, i64, f64, String, String)>()?;

    // 3. CanSqliteJson<AppCodec> on `Column<AuditPayload>` and `Column<Option<AuditPayload>>` + SRFs
    let _json_rows = client
        .select::<EventLog>()
        .srf((
            JsonEach(EventLog::payload),
            JsonTree((EventLog::opt_payload, "$.nested")),
        ))
        .columns((
            Json(EventLog::payload),
            Json(EventLog::opt_payload),
            EventLog::payload.json_extract("$.actor"),
            EventLog::opt_payload.jsonb_extract("$.level"),
        ))
        .filter(
            EventLog::payload.json_extract("$.actor").eq("admin")
                & EventLog::opt_payload.jsonb_extract("$.level").gt(1_i32),
        )
        .load_in::<(String, String, String, i32)>()?;

    Ok(())
}
