#![allow(unreachable_code, dead_code, unused)]

use kova::dsl::func::Count;
use kova::query::Select;
use kova::types::Engine;
use kova_core::database::Executor;
use kova_core::dsl::Column;
use kova_core::types::Select as CoreSelect;
use kova_core::types::{FromValue, IntoValue};

#[path = "../common/models.rs"]
mod models;

use models::{Order, User};

// ==========================================
// Stress Test 1: The ultimate chain of all syntax nodes
// ==========================================
fn compile_check_sqlite_monster_query() {
    let executor: Executor<kova_sqlite::types::SqlitePool> = unimplemented!();

    let _query = Select::<'_, '_, User, _, _>::new(&executor)
        // 1. Stress test IntoArgs<Projection> polymorphic tuple: standard column + aggregate function + alias extension
        .columns(User::name)
        .columns((
            User::id,
            Order::amount,
            Count(User::id).alias(User::id),
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
        // 5. Stress test Having boundaries: SQLite lacks ArrayAgg, using standard aggregate functions to test LHS
        .having(Count(Order::id).gt(100_i32))
        // 6. Stress test Order By polymorphic tuple: explicit modifiers + implicit derivation
        .order_by(User::name.asc())
        .order_by((
            Order::amount.desc().nulls_last(),
            User::name.asc(),
            User::id,
        ))
        // 7. Stress test Pagination extensions (SQLite natively supports limit and offset)
        .limit(50)
        .offset(100);
    // Note: Removed PG-specific .lock() and ArrayAgg tests
}

// ==========================================
// Stress Test 2: Extraction limits of the Load API (SQLite Sync version)
// ==========================================
// SQLite's ConnectionPool and Executable use Sync mode, so async/await is removed
fn compile_check_sqlite_load_endpoints() -> kova_core::Result<()> {
    let executor: Executor<kova_sqlite::types::SqlitePool> = unimplemented!();

    // Endpoint 1: Load main model
    let query = Select::<'_, '_, User, _, _>::new(&executor);
    let _users: Vec<User> = query.load()?;

    // Endpoint 2: Map to another model (Verify U: FromRow)
    let query = Select::<'_, '_, User, _, _>::new(&executor);
    let _orders: Vec<Order> = query.load_as::<Order>()?;

    let query = Select::<'_, '_, User, _, _>::new(&executor);
    let _orders: Vec<(User, Order)> = query.load_as::<(User, Order)>()?;

    // Endpoint 3: Extract scalar (Single column, triggers basic implementation of DecodeColumn)
    let query = Select::<'_, '_, User, _, _>::new(&executor);
    let _ids: Vec<i32> = query.load_in::<i32>()?;

    // Endpoint 4: Extract single column Option
    let query = Select::<'_, '_, User, _, _>::new(&executor);
    let _optional_amounts: Vec<Option<f64>> = query.load_in::<Option<f64>>()?;

    // Endpoint 5: Extract giant strongly-typed Tuple (Triggers bind_decode_columns_tuple! macro limits)
    let query = Select::<'_, '_, User, _, _>::new(&executor);
    let _complex_rows: Vec<(i32, String, Option<f64>, bool, Vec<u8>)> = query
        .load_in::<(i32, String, Option<f64>, bool, Vec<u8>)>()?;

    Ok(())
}