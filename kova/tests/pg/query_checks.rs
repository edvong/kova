#![allow(unreachable_code, dead_code, unused)]

use kova::dsl::func::{ArrayAgg, Count};
use kova::dsl::{LockMode, WaitPolicy};
use kova::query::Select;
use kova::types::Engine;
use kova_core::database::Executor;
use kova_core::dsl::Column;
use kova_core::dsl::func::{Avg, CountAll};
use kova_core::types::Select as CoreSelect;
use kova_pg::types::{PgPool, Postgres};

#[path = "../common/models.rs"]
mod models;

use models::{Order, User};

/// Stress Test 1: The ultimate chain of all syntax nodes
fn compile_check_pg_monster_query() {
    let executor: Executor<PgPool> = unimplemented!();

    let a = Order::is_active.eq::<_, ()>(false);

    let _query = Select::<'_, '_, Postgres, User, PgPool, ()>::new(&executor)
        // 1. Stress test IntoArgs<Projection> polymorphic tuple: standard column + aggregate function + alias extension + constant
        .columns(User::name)
        .columns((
            User::id,
            Order::amount,
            Count(User::id).alias(User::id), // Core function wrapper
            ArrayAgg(Order::amount)
                .distinct()
                .order_by(Order::id.desc().nulls_first())
                .filter(
                    Order::is_active.eq(true)
                    & Order::amount.gt(100_f64)
                )
                .alias(User::name), // Forcefully borrow alias to verify type boundaries
        ))
        // 2. Stress test JOIN boundaries: generic Model parameters + Column vs Column comparison
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .left_join::<Order>(Order::is_active.ne(false)) // Column vs Value
        // 3. Stress test BitAnd (&) / BitOr (|) nesting and all condition operators
        .filter(
            User::name.like("%admin%")
                & (Order::amount.between(10.5_f64, 99.9_f64)
                | Order::id.in_list(vec![1, 2, 3])
                | Order::raw_data.is_not_null()),
        )
        // 4. Stress test Group By polymorphic tuple: mixed grouping of columns + functions
        .group_by(User::id)
        .group_by((User::id, Order::is_active))
        // 5. Stress test Having boundaries: using extension function as left-hand side (LHS) operand
        .having(Avg(User::id).eq(100))
        .having(ArrayAgg(Order::id).gt(100_i32))
        // 6. Stress test Order By polymorphic tuple: explicit modifiers + implicit derivation
        .order_by(User::name.asc())
        .order_by((
            Order::amount.desc().nulls_last(),
            User::name.asc(),
            User::id, // Implicitly converted to OrderOperand without modifiers
        ))
        // 7. Stress test Pagination extensions
        .limit(50)
        .offset(100)
        // 8. Stress test Locking extensions
        .lock::<User>(LockMode::NoKeyUpdate, Some(WaitPolicy::NoWait))
        .lock_all(LockMode::Share, None);
}

/// Stress Test 2: Lifecycle of Load API and extraction limits of macro-generated Tuples
async fn compile_check_pg_load_endpoints() -> kova_core::Result<()> {
    let executor: Executor<PgPool> = unimplemented!();

    let query = Select::<'_, '_, Postgres, User, PgPool, ()>::new(&executor);

    // Endpoint 1: Load main model
    let _users: Vec<User> = query.load().await?;

    // Endpoint 2: Map to another model (Verify U: FromRow)
    let query = Select::<'_, '_, Postgres, User, PgPool, ()>::new(&executor);
    let _orders: Vec<Order> = query.load_as::<Order>().await?;
    let _orders: Vec<(User, Order)> = query.load_as::<(User, Order)>().await?;

    // Endpoint 3: Extract scalar (Single column, triggers basic implementation of DecodeColumn)
    let query = Select::<'_, '_, Postgres, User, PgPool, ()>::new(&executor);
    let _ids: Vec<i32> = query.load_in::<i32>().await?;

    // Endpoint 4: Extract single column Option
    let query = Select::<'_, '_, Postgres, User, PgPool, ()>::new(&executor);
    let _optional_amounts: Vec<Option<f64>> = query.load_in::<Option<f64>>().await?;

    // Endpoint 5: Extract giant strongly-typed Tuple (Triggers bind_decode_columns_tuple! macro limits)
    let query = Select::<'_, '_, Postgres, User, PgPool, ()>::new(&executor);
    let _complex_rows: Vec<(i32, String, Option<f64>, bool, Vec<u8>)> = query
        .load_in::<(i32, String, Option<f64>, bool, Vec<u8>)>()
        .await?;

    Ok(())
}