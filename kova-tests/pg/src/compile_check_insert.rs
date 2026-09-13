use crate::Result;
use crate::models::user::User;
use kova::Options;
use kova::dsl::func::Lower;

/// Stress Test 3: Insert Statements & Conflict Handling
async fn compile_check_pg_insert() -> Result<()> {
    let client = Options::from("").build()?;

    // Mocking data references
    let user = User::default();
    let users = vec![User::default(), User::default()];

    // 1. Single & Batch Execution (No Return)
    client.insert(user.clone()).execute().await?;
    client.insert(users.clone()).execute().await?;

    // 2. Insert Returning with diverse target tuples
    let _ids = client
        .insert(&user.clone())
        .returning(User::id)
        .execute_in::<i32>()
        .await?;

    let _rows = client
        .insert(&users)
        .returning((User::id, User::name))
        .execute_in::<(i32, String)>()
        .await?;

    let _models = client
        .insert(&users)
        .returning((User::id, User::name))
        .execute_as::<User>() // Requires FromRow mapping
        .await?;

    // 3. On Conflict Do Nothing (With single target)
    client
        .insert(user.clone())
        .on_conflict(User::id)
        .do_nothing()
        .execute()
        .await?;

    // 4. On Conflict Update (With multiple targets, functions, and EXCLUDED assignments)
    client
        .insert(&users.clone())
        .on_conflict((User::id, Lower(User::name)))
        .update((
            User::name.set("updated_name"),
            User::is_active.excluded(), // Triggers EXCLUDED.is_active internally
        ))
        .execute()
        .await?;

    Ok(())
}