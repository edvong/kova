use crate::Result;
use crate::type_check::codec::AppCodec;
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::user::User;
use chrono::{DateTime, Utc};
use kova::Options;
use kova::dsl::func::{
    Count, GenerateSeries, JsonbArrayElements, JsonbEach, JsonbEachText, Unnest,
};
use kova::query::Select;
use models::type_check::post::Post;

struct DateSeries;
struct KvAlias;

/// Stress Test 1: Array Expansion using Unnest (Vec & Column)
async fn compile_check_srf_unnest() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let bulk_ids: Vec<i32> = vec![1, 2, 3, 4, 5];
    let unnest_fn = Unnest(bulk_ids);

    let _ = client
        .select::<User>()
        .srf(unnest_fn)
        .columns((
            User::id,
            User::name,
            Unnest::unnest, // Static schema access
        ))
        .filter(User::id.eq(Unnest::unnest))
        .load_in::<(i32, String, i32)>()
        .await?;

    let id_slice: &[i32] = &[10, 20, 30];
    let fixed_ids: [i32; 3] = [1, 2, 3];
    let copyable_srf = Unnest(Post::tags);

    // Passing `&copyable_srf` by reference (since `Column` is `Copy`) and accessing `.unnest` via Deref
    let _ = client
        .select::<Post>()
        .srf((&copyable_srf, Unnest(id_slice), Unnest(fixed_ids), Unnest(&fixed_ids)))
        .columns((Post::id, copyable_srf.unnest))
        .filter(copyable_srf.unnest.eq("rust"))
        .load_in::<(i32, String)>()
        .await?;

    Ok(())
}

/// Stress Test 2: Deep JSONB Parsing, Deref Schema Access & Multi-SRF Injection
async fn compile_check_srf_jsonb() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let each_fn = JsonbEachText(EventLog::metadata);

    let _ = client
        .select::<EventLog>()
        .srf((
            JsonbArrayElements(EventLog::metadata.json_path("tags")),
            JsonbEachText(EventLog::metadata),
        ))
        .tag::<KvAlias>()
        .columns((
            EventLog::id,
            JsonbArrayElements::value,
            // Testing both static associated const and Deref instance field access
            JsonbEachText::key.tag::<KvAlias>(),
            each_fn.value.tag::<KvAlias>(),
        ))
        .filter(
            JsonbArrayElements::value.eq("urgent")
                & each_fn.key.tag::<KvAlias>().eq("source"),
        )
        .load_in::<(uuid::Uuid, String, String, String)>()
        .await?;

    Ok(())
}

/// Stress Test 3: GenerateSeries (With Step & Without Step)
async fn compile_check_srf_generate_series() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    // Scenario A: Without .step() (verifies phantom S = L binding)
    let _ = client
        .select::<User>()
        .srf(GenerateSeries(1_i32, 100_i32))
        .filter(User::id.eq(GenerateSeries::generate_series))
        .load_as::<User>()
        .await?;

    // Scenario B: With .step() and heterogeneous timestamp/interval literals
    let _ = client
        .select::<Order>()
        .srf(GenerateSeries("2026-01-01", "2026-12-31").step("1 day"))
        .tag::<DateSeries>()
        .left_join::<Order>(
            Order::created_at.eq(GenerateSeries::generate_series.tag::<DateSeries>()),
        )
        .columns((
            GenerateSeries::generate_series.tag::<DateSeries>(),
            Count(Order::id),
        ))
        .group_by(GenerateSeries::generate_series.tag::<DateSeries>())
        .order_by(GenerateSeries::generate_series.tag::<DateSeries>().desc())
        .load_in::<(DateTime<Utc>, i64)>()
        .await?;

    Ok(())
}

/// Stress Test 4: Subqueries inside SRFs
async fn compile_check_srf_subquery_injection() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let _ = client
        .select::<User>()
        .filter(
            User::id.in_select(
                Select::from::<Order>()
                    .srf(JsonbEach(
                        Select::from::<EventLog>()
                            .columns(EventLog::metadata)
                            .filter(EventLog::id.is_not_null()),
                    ))
                    .columns(JsonbEach::value),
            ),
        )
        .load_as::<User>()
        .await?;

    Ok(())
}