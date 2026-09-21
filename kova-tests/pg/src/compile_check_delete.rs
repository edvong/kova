use crate::Result;
use crate::models::order::Order;
use crate::models::user::User;
use kova::Options;

/// Stress Test 5: Delete Statements
async fn compile_check_delete() -> Result<()> {
    let client = Options::from("").build()?;

    // 1. Standard Delete
    client.delete::<User>().filter(User::id.eq(1)).execute().await?;

    // 2. Delete with complex filter tree
    client
        .delete::<Order>()
        .filter(Order::amount.is_null() | Order::is_active.eq(false))
        .execute()
        .await?;

    // 3. Delete Returning Model
    let _deleted_users = client
        .delete::<User>()
        .filter(User::name.like("%test%"))
        .returning((User::id, User::name))
        .execute_as::<User>()
        .await?;

    Ok(())
}
