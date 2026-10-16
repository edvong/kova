#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::models::order::Order;
use crate::models::user::User;
use kova::Options;
use kova::dsl::Column;
use kova::dsl::func::{
    Avg, Count, FrameBound, GroupConcat, JsonGroupArray, Sum, Wildcard, Window,
};
use kova::query::Select;
use kova::types::{Cte, IdentifierMeta, Relation};

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

/// Stress Test 1: Standard & SQLite-Specific Aggregates with Window Clauses and Frame Bounds
fn compile_check_sqlite_window_functions() -> Result<()> {
    let client = Options::in_memory().build()?;

    let _ = client
        .select::<Order>()
        .columns((
            Order::id,
            Order::user_id,
            // 1. Basic partition agg: COUNT(*) OVER (PARTITION BY user_id)
            Count(Wildcard)
                .over(Window().partition_by(Order::user_id))
                .alias(TopSpender::order_count),
            // 2. Rolling sum with ROWS frame bound
            Sum(Order::amount)
                .over(
                    Window()
                        .partition_by(Order::user_id)
                        .order_by(Order::id.asc())
                        .rows(FrameBound::UnboundedPreceding, Some(FrameBound::CurrentRow)),
                )
                .alias(TopSpender::total_amount),
            // 3. Moving average with RANGE frame bound
            Avg(Order::amount).over(
                Window()
                    .partition_by(Order::user_id)
                    .order_by(Order::amount.asc())
                    .range(FrameBound::Preceding(1), Some(FrameBound::Following(1))),
            ),
            // 4. SQLite GroupConcat and JsonGroupArray used as window functions with GROUPS bound
            GroupConcat(Order::id)
                .separator("|")
                .over(
                    Window()
                        .partition_by(Order::user_id)
                        .order_by(Order::id.asc())
                        .groups(FrameBound::CurrentRow, Some(FrameBound::UnboundedFollowing)),
                ),
            JsonGroupArray(Order::amount)
                .filter(Order::is_active.eq(true))
                .over(Window().partition_by((Order::user_id, Order::is_active))),
        ))
        .load_in::<(i32, i32, i64, f64, f64, String, String)>()?;

    Ok(())
}

/// Stress Test 2: CTE & Subselect Integration on SQLite
fn compile_check_sqlite_cte_and_subquery() -> Result<()> {
    let client = Options::in_memory().build()?;

    let _ = client
        .select::<User>()
        .with::<TopSpender>(
            Select::from::<Order>()
                .columns((
                    Order::user_id,
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
        .load_in::<(i32, String, f64, i64)>()?;

    Ok(())
}
