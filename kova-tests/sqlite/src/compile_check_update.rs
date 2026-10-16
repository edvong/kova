use crate::models::order::Order;
use crate::models::user::User;
use kova::Options;

/// Stress Test 4: Update Statements, Increment/Decrement & Cross-Column Assignments
fn compile_check_sqlite_update() -> crate::Result<()> {
    let client = Options::in_memory().build()?;

    // 1. Single Column Update + Increment / Decrement
    client
        .update::<User>()
        .set(User::name.set("new_name"))
        .set(User::id.increment(1))
        .set(User::balance.decrement(5.5))
        .filter(User::id.eq(1))
        .execute()?;

    // 2. Multi-column Update & Column-to-Column assignments
    client
        .update::<Order>()
        .set((
            Order::amount.set(99.9_f64),
            Order::is_active.set(true),
            Order::user_id.set(Order::id),
        ))
        .filter(Order::user_id.in_list(vec![1, 2, 3]))
        .execute()?;

    // 3. Update Returning Tuple
    let _updated_records = client
        .update::<Order>()
        .set(Order::is_active.set(false))
        .filter(Order::amount.lt(10.0_f64))
        .returning((Order::id, Order::amount))
        .execute_in::<(i64, f64)>()?;

    Ok(())
}
