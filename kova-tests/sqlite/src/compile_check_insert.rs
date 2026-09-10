use kova::dsl::func::Lower;
use kova::Options;
use crate::models::User;

/// Stress Test 3: Insert Statements & Conflict Handling Boundaries
fn compile_check_sqlite_insert() -> crate::Result<()> {
    let client = Options::in_memory().build()?;

    let user = User::default();
    let users = vec![User::default(), User::default()];

    // 1. Single & Batch Execution (No Return)
    client.insert(user.clone()).execute()?;

    // Testing boundary of passing empty batches
    client.insert(Vec::<User>::new()).execute()?;

    // 2. Insert Returning with diverse target tuples
    let _ids = client
        .insert(&user)
        .returning(User::id)
        .execute_in::<i32>()?;

    let _rows = client
        .insert(&users)
        .returning((User::id, User::name))
        .execute_in::<(i32, String)>()?;

    let _models = client
        .insert(&users)
        .returning((User::id, User::name))
        .execute_as::<User>()?;

    // 3. On Conflict Do Nothing (With multiple tuple targets)
    client
        .insert(user.clone())
        .on_conflict((User::id, User::name))
        .do_nothing()
        .execute()?;

    // 4. On Conflict Update (With Function conflict targets and Self-referencing exclusion)
    client
        .insert(&users)
        .on_conflict((User::id, Lower(User::name)))
        .update((
            User::name.set("updated_name"),
            User::is_active.excluded(),
        ))
        .execute()?;

    Ok(())
}
