#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::models::order::Order;
use crate::models::user::User;
use kova::Options;
use kova::dsl::func::{Abs, Avg, Count, CountAll, Length, Lower, Max, Min, Sum, Upper};

/// Stress Test 1: Select Statements & Deep AST Boundaries
fn compile_check_sqlite_select() {
    let client = Options::in_memory().build().unwrap();

    client
        .select::<User>()
        // 1. Stress test Tuples up to large sizes, mixed scalar, function, alias, and core functions
        .columns(CountAll)
        .columns(User::name)
        .columns((
            User::id,
            Order::amount,
            Count(User::id).alias(User::id),
            Sum(Order::amount).alias(User::name),
            Min(Order::amount),
            Max(Order::amount),
            Avg(Order::amount),
            Lower(User::name),
            Upper(User::name),
            Abs(Order::amount),
            Length(User::name),
        ))
        // 2. JOIN boundaries: Model to Model, and Column to Value comparison
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .left_join::<Order>(Order::is_active.ne(false))
        // 3. Deep Boolean Logic Boundaries:
        // Testing left-deep and right-deep trees, mixed precedence, and zero-copy byte slice references (&[u8])
        .filter(
            (User::name.like("%admin%") & Order::amount.is_not_null())
                | (
                Order::amount.between(10.5_f64, i64::MAX as f64) // Boundary type casting
                    & Order::amount.not_between(0.0_f64, 1.0_f64)
                    & Order::id.in_list(vec![1, 2, 3])
                    & Order::id.not_in_list(vec![4, 5, 6])
            )
                | (
                Order::amount.ge(0.0_f64)
                    | Order::amount.le(f64::MAX)
                    | Order::raw_data.eq(&b"zero_copy_bytes"[..]) // Zero-copy blob bound check
            )
        )
        // 4. Group By Boundaries: Multiple mixed items
        .group_by(User::id)
        .group_by(Order::is_active)
        .group_by(Lower(User::name))
        .group_by((User::id, Order::is_active, Lower(User::name)))
        // 5. Having Boundaries: Aggregates evaluated against dynamic targets
        .having(Sum(Order::amount).gt(1000.50_f64))
        // 6. Order By Boundaries: Multiple Modifiers and Implicit Nulls
        .order_by((
            Order::amount.desc().nulls_last(),
            User::name.asc().nulls_first(),
            User::id,
        ))
        // 7. Pagination Bounds
        .limit(usize::MAX)
        .offset(0);
}

/// Stress Test 2: Lifecycle of Load API (Synchronous)
fn compile_check_sqlite_load_endpoints() -> Result<()> {
    let client = Options::in_memory().build()?;

    // Endpoint 1: Load main model
    let _users = client.select::<User>().load()?;

    // Endpoint 2: Map to another model / Join Tuples
    let _orders = client.select::<Order>().load()?;
    let _orders = client.select::<User>().load_as::<(User, Order)>()?;

    // Endpoint 3: Extract single column (Nullable)
    let _ids = client.select::<User>().load_in::<Option<f64>>()?;

    // Endpoint 4: Extract giant strongly-typed Tuple mapping to native SQLite Int8/Float8/Blob
    let _complex_rows = client
        .select::<User>()
        .load_in::<(i64, String, Option<f64>, bool, Vec<u8>)>()?;

    Ok(())
}
