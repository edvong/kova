use kova::Options;
use crate::models::order::Order;
use crate::models::user::User;

/// Stress Test 5: Delete Statements
fn compile_check_sqlite_delete() -> crate::Result<()> {
    let client = Options::in_memory().build()?;

    // 1. Standard Delete
    client
        .delete::<User>()
        .filter(User::id.eq(1))
        .execute()?;

    // 2. Delete with complex filter tree & explicit Option checks
    client
        .delete::<Order>()
        .filter(
            Order::amount.is_null()
                | Order::is_active.eq(false)
                | Order::raw_data.is_not_null()
        )
        .execute()?;

    // 3. Delete Returning Model
    let _deleted_users = client
        .delete::<User>()
        .filter(User::name.like("%test%"))
        .returning((User::id, User::name))
        .execute_as::<User>()?;

    Ok(())
}