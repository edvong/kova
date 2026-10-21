use kova::Options;
use models::type_check::user::User;

fn main() {
    let client = Options::in_memory().build().unwrap();
    let user = User::default();

    // 1. Update in Unset state cannot call .execute(), .execute_in(), .execute_as(), or .returning()
    let _ = client.update::<User>().filter(User::id.eq(1)).execute();
    let _ = client.update::<User>().execute_in::<i32>();
    let _ = client.update::<User>().returning(User::id);

    // 2. NoReturn state cannot call execute_as or execute_in
    let _ = client.insert(&user).execute_in::<i32>();
    let _ = client.update::<User>().set(User::name.set("a")).execute_as::<User>();
    let _ = client.delete::<User>().execute_in::<i32>();

    // 3. Returning state cannot call .execute()
    let _ = client.insert(&user).returning(User::id).execute();
    let _ = client
        .update::<User>()
        .set(User::name.set("a"))
        .returning(User::id)
        .execute();
    let _ = client.delete::<User>().returning(User::id).execute();

    // 4. Cannot call .returning() twice, and Update cannot call .set() after .returning()
    let _ = client.insert(&user).returning(User::id).returning(User::name);
    let _ = client
        .update::<User>()
        .set(User::name.set("a"))
        .returning(User::id)
        .returning(User::name);
    let _ = client
        .update::<User>()
        .set(User::name.set("a"))
        .returning(User::id)
        .set(User::is_active.set(true));
    let _ = client.delete::<User>().returning(User::id).returning(User::name);

    // 5. Insert conflict state machine:
    // - NoConflict cannot call .do_nothing(), .update(), or .filter()
    let _ = client.insert(&user).do_nothing();
    let _ = client.insert(&user).update(User::name.set("a"));
    let _ = client.insert(&user).filter(User::is_active.eq(true));

    // - ConflictTarget (incomplete on_conflict without action) cannot execute or filter
    let _ = client.insert(&user).on_conflict(User::id).execute();
    let _ = client
        .insert(&user)
        .on_conflict(User::id)
        .returning(User::id)
        .execute_in::<i32>();
    let _ = client
        .insert(&user)
        .on_conflict(User::id)
        .filter(User::is_active.eq(true));

    // - ConflictNothing cannot call .filter() or transition to another conflict action
    let _ = client
        .insert(&user)
        .on_conflict(User::id)
        .do_nothing()
        .filter(User::is_active.eq(true));
    let _ = client
        .insert(&user)
        .on_conflict(User::id)
        .do_nothing()
        .update(User::name.set("a"));

    // - Cannot call .on_conflict() twice
    let _ = client
        .insert(&user)
        .on_conflict(User::id)
        .on_conflict(User::name);
}