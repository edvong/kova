#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::models::order::Order;
use crate::models::user::User;
use kova::Options;
use kova::dsl::Column;
use kova::dsl::func::{
    Count, CumeDist, DenseRank, FrameBound, Lag, Lead, Ntile, PercentRank, Rank, RowNumber, Sum,
    Wildcard, Window,
};
use kova::query::Select;
use kova::types::{Cte, IdentifierMeta, Relation};

// ==========================================
// Virtual Table Definition (Simulating #[derive(Cte)])
// ==========================================
pub struct TopSpender;

impl Relation for TopSpender {
    const RELATION_NAME: &'static str = "top_spender";
}
impl Cte for TopSpender {}

#[allow(non_upper_case_globals)]
impl TopSpender {
    pub const user_id: Column<i32, IdentifierMeta> =
        Column::identifier("top_spender", "user_id");
    pub const total_amount: Column<f64, IdentifierMeta> =
        Column::identifier("top_spender", "total_amount");
    pub const order_count: Column<i64, IdentifierMeta> =
        Column::identifier("top_spender", "order_count");
}

/// Stress Test 1: Standard Aggregates with Window Clauses
async fn compile_check_window_functions() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<Order>()
        .columns((
            Order::id,
            Order::user_id,
            // Scenario A: Basic partition agg (COUNT(*) OVER (PARTITION BY user_id))
            Count(Wildcard)
                .over(Window().partition_by(Order::user_id))
                .alias(TopSpender::order_count),
            // Scenario B: Partition + Order for rolling accumulation
            Sum(Order::amount)
                .over(Window().partition_by(Order::user_id).order_by(Order::id.asc()))
                .alias(TopSpender::total_amount),
            // Scenario C: Multi-field compound partition without alias
            Count(Order::id).over(Window().partition_by((Order::user_id, Order::is_active))),
        ))
        .load_in::<(i32, i32, i64, f64, i64)>()
        .await?;

    Ok(())
}

/// Stress Test 2: CTE & Subselect Integration
async fn compile_check_cte_and_subquery() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<User>()
        .with::<TopSpender>(
            Select::from::<Order>()
                .columns((
                    Order::user_id,
                    Sum(Order::amount).alias(Order::amount),
                    Sum(Order::amount).alias(TopSpender::total_amount),
                    Count(Wildcard).alias(TopSpender::order_count),
                ))
                .group_by(Order::user_id)
                .having(Sum(Order::amount).gt(100.0_f64)),
        )
        .columns((
            User::id,
            User::name,
            TopSpender::total_amount,
            TopSpender::order_count,
        ))
        .inner_join::<TopSpender>(User::id.eq(TopSpender::user_id))
        .filter(TopSpender::order_count.ge(5) & TopSpender::total_amount.is_not_null())
        .order_by(TopSpender::total_amount.desc())
        .load_in::<(i32, String, f64, i64)>()
        .await?;

    Ok(())
}

/// Stress Test 3: PG Exclusive Window Functions & Frame Bounds
async fn compile_check_exclusive_window_functions() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<Order>()
        .columns((
            Order::id,
            Order::user_id,
            Order::amount,
            // Parameterless ranking window functions -> i64
            RowNumber().over(Window().partition_by(Order::user_id).order_by(Order::amount.desc())),
            Rank().over(Window().partition_by(Order::user_id).order_by(Order::amount.desc())),
            DenseRank().over(Window().partition_by(Order::user_id).order_by(Order::amount.desc())),
            // Distribution window functions -> f64
            PercentRank().over(Window().partition_by(Order::user_id).order_by(Order::amount.asc())),
            CumeDist().over(Window().partition_by(Order::user_id).order_by(Order::amount.asc())),
            // Bucketing window function (accepts literal or AST node) -> i32
            Ntile(4).over(Window().partition_by(Order::user_id).order_by(Order::amount.desc())),
            // Offset window functions: literal offset + literal default, and AST column default
            Lag(Order::amount)
                .offset(1)
                .default(0.0_f64)
                .over(Window().partition_by(Order::user_id).order_by(Order::id.asc())),
            Lead(Order::amount)
                .offset(Order::id)
                .default(Order::amount)
                .over(Window().partition_by(Order::user_id).order_by(Order::id.asc())),
            // Frame Bounds: rows, range, groups
            Sum(Order::amount).over(
                Window()
                    .partition_by(Order::user_id)
                    .order_by(Order::id.asc())
                    .rows(FrameBound::UnboundedPreceding, Some(FrameBound::CurrentRow)),
            ),
            Count(Order::id).over(
                Window()
                    .partition_by(Order::user_id)
                    .order_by(Order::amount.asc())
                    .range(FrameBound::Preceding(1), Some(FrameBound::Following(1))),
            ),
            Sum(Order::amount).over(
                Window()
                    .partition_by(Order::user_id)
                    .order_by(Order::id.asc())
                    .groups(FrameBound::CurrentRow, Some(FrameBound::UnboundedFollowing)),
            ),
        ))
        .load_in::<(i32, i32, f64, i64, i64, i64, f64, f64, i32, f64, f64, f64, i64, f64)>()
        .await?;

    Ok(())
}