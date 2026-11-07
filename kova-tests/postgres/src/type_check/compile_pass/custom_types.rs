use crate::Result;
use crate::type_check::codec::AppCodec;
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Options;
use kova::dsl::LockMode;
use kova::dsl::func::{
    Abs, Avg, JsonbArrayElements, JsonbEach, JsonbEachText, JsonbObject, Length, Lower, Sum,
    Unnest, Upper,
};
use kova::dsl::func::{Count, Wildcard};
use models::type_check::event_log::{AuditPayload, Decimal, EventLog, TagList};
use models::type_check::post::Post;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Stress Test: Custom Types, Reference Penetration, and Custom Compatible Rules
async fn compile_check_custom_types() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

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

    client.insert(&dummy_log).execute().await?;

    // 1. Upsert + Single custom scalar extraction (requires bind_type! for bare Uuid)
    let _upsert_id = client
        .insert(&dummy_log)
        .on_conflict(EventLog::id)
        .update((EventLog::updated_at.set(Timestamp::now()), EventLog::metadata.excluded()))
        .returning(EventLog::id)
        .execute_in::<Uuid>()
        .await?;

    let mut new_meta = HashMap::new();
    new_meta.insert("archived".into(), "true".into());

    // 2. Update with owned values, references (&target_uuid), and custom Compatible (&str -> Url)
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
        .execute_in::<Url>()
        .await?;

    // 3. Delete + Returning bare DateTime<Utc>
    let _deleted_time = client
        .delete::<EventLog>()
        .filter(EventLog::website.eq(&site) | EventLog::website.eq("https://old.site"))
        .returning(EventLog::created_at)
        .execute_in::<DateTime<Utc>>()
        .await?;

    // 4. Select with value vs reference and between/in_list on custom types
    let _logs = client
        .select::<EventLog>()
        .filter(
            EventLog::id.in_list([&target_uuid, &Uuid::nil()])
                & EventLog::created_at.between(&current_time, Utc::now()),
        )
        .load_as::<EventLog>()
        .await?;

    // 5. Heterogeneous tuple extraction + CountAll() + pessimistic locking
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
        .lock_all(LockMode::Update, None)
        .load_in::<(Uuid, Url, DateTime<Utc>, Timestamp, i64)>()
        .await?;

    // 6. Option<CustomType> and Box<CustomType> extraction (handled generically by kova-kernel)
    let _optional_urls =
        client.select::<EventLog>().columns(EventLog::website).load_in::<Option<Url>>().await?;

    let _boxed_uuids =
        client.select::<EventLog>().columns(EventLog::id).load_in::<Box<Uuid>>().await?;

    let _extracted_maps = client
        .select::<EventLog>()
        .columns((EventLog::website, EventLog::metadata))
        .load_in::<(Option<Url>, HashMap<String, String>)>()
        .await?;

    Ok(())
}

/// Stress Test 2: Codec-Aware Domain Capabilities (`CanMath<X>`, `CanString<X>`, `CanPgJsonb<X>`, `CanPgArray<X>`)
async fn compile_check_custom_domain_capabilities() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let base_fee = Decimal(100);
    let audit = AuditPayload { actor: "system".into(), level: 3 };
    let tags = TagList(vec!["core".into(), "db".into()]);
    let raw_tag_slice: &[&str] = &["rust", "postgres"];

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
        .load_in::<(Decimal, Option<Decimal>, Decimal, Option<Decimal>, Decimal)>()
        .await?;

    client
        .update::<EventLog>()
        .set((
            EventLog::cost.increment(50_i64),
            EventLog::cost.decrement(&base_fee),
            EventLog::opt_cost.increment(Decimal(10)),
        ))
        .execute()
        .await?;

    // 2. CanString<AppCodec> (and blanket CanPgString<AppCodec>) on `Column<Url>` and `Column<Option<Url>>`
    let _str_rows = client
        .select::<EventLog>()
        .columns((
            Lower(EventLog::website),
            Upper(EventLog::opt_website),
            Length(EventLog::website),
        ))
        .filter(
            EventLog::website.like("https://%")
                & EventLog::opt_website.not_like("http://%")
                & EventLog::website.ilike("HTTPS://%.ORG")
                & EventLog::opt_website.not_ilike("%tracking%")
                & EventLog::website.matches("^https://[a-z]+")
                & EventLog::opt_website.imatches("rust-lang")
                & Length(EventLog::website).gt(12_i64),
        )
        .load_in::<(String, String, i64)>()
        .await?;

    // 3. CanPgJsonb<AppCodec> on `Column<AuditPayload>` and `Column<Option<AuditPayload>>` + SRFs
    let _json_rows = client
        .select::<EventLog>()
        .srf((
            JsonbEach(EventLog::payload),
            JsonbEachText(EventLog::opt_payload),
            JsonbArrayElements(EventLog::payload.json_path("history")),
        ))
        .filter(
            EventLog::payload.json_path("actor").json_contains(&audit)
                & EventLog::opt_payload.json_path_text("actor").eq("admin")
                & EventLog::payload.json_has_key("level")
                & EventLog::opt_payload.json_has_all_keys(["actor", "level"])
                & EventLog::payload.json_path_exists("$.level ? (@ > 1)"),
        )
        .load_as::<EventLog>()
        .await?;

    // 4. CanPgArray<AppCodec> on `Column<TagList>` and `Column<Option<TagList>>` + Unnest + JsonbObject
    let _array_rows = client
        .select::<EventLog>()
        .srf(Unnest(EventLog::labels))
        .inner_join::<Post>(Post::id.eq(1))
        .columns((
            Unnest::unnest,
            JsonbObject(EventLog::labels, Post::tags),
        ))
        .filter(
            EventLog::labels.array_contains(&tags)
                & EventLog::labels.array_overlap(raw_tag_slice)
                & EventLog::opt_labels.array_contained_by(raw_tag_slice),
        )
        .load_in::<(String, AuditPayload)>()
        .await?;

    Ok(())
}