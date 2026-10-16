#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::codec::conversion::AppCodec;
use crate::codec::models::EventLog;
use crate::models::order::Order;
use crate::models::user::User;
use chrono::{DateTime, Utc};
use kova::Options;
use kova::dsl::func::{
    Count, GenerateSeries, JsonbArrayElements, JsonbEach, JsonbEachText, Unnest,
};
use kova::query::Select;

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
    let client = Options::from("").build()?;

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