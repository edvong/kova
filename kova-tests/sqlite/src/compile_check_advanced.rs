#![allow(unreachable_code, dead_code, unused)]

use kova::dsl::Subselect;
use crate::Result;
use crate::codec::models::EventLog;
use crate::models::comment::Comment;
use crate::models::order::Order;
use crate::models::post::Post;
use crate::models::user::User;
use kova::Options;

/// Test 1: Polymorphic Joins (Seamlessly switching between ON and USING)
fn compile_check_advanced_joins() -> Result<()> {
    let client = Options::in_memory().build()?;

    let _ = client
        .select::<Order>()
        .inner_join::<User>(Order::user_id.eq(User::id) | Order::amount.gt(100.0_f64))
        .left_join::<Post>(Post::author_id)
        .right_join::<Comment>((Comment::user_id, Comment::post_id))
        .full_join::<EventLog>(EventLog::description.like("%warn%"))
        .cross_join::<User>()
        .load_as::<Order>()?;

    Ok(())
}

/// Test 2: Upsert Where (Boundary validation for secondary filtering on conflict)
fn compile_check_upsert_where() -> Result<()> {
    let client = Options::in_memory().build()?;
    let users = vec![User::default(), User::default()];

    client
        .insert(&users)
        .on_conflict((User::id, User::name))
        .update((
            User::name.set("conflict_resolved"),
            User::is_active.set(true),
        ))
        .filter(User::is_active.eq(false))
        .execute()?;

    Ok(())
}

/// Test 3: Update From (Cross-table update support)
fn compile_check_update_from() -> Result<()> {
    let client = Options::in_memory().build()?;

    client
        .update::<Order>()
        .set(Order::amount.set(999.9_f64))
        .from::<User>()
        .from::<(User, Post)>()
        .filter(
            Order::user_id.eq(User::id)
                & User::id.eq(Post::author_id)
                & User::is_active.eq(true)
        )
        .execute()?;

    Ok(())
}

/// Test 4: Set Operation
fn compile_check_set_operations() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<User>()
        // Main query: extracting id and name
        .columns((User::id, User::name))
        .filter(User::is_active.eq(true))

        // Tacking on UNION ALL
        .union_all(
            Subselect::<User>::columns((User::id, User::name))
                .filter(User::is_active.eq(false))
        )
        // Chaining EXCEPT
        .except(
            Subselect::<User>::columns((User::id, User::name))
                .filter(User::name.like("%test%"))
        )
        // Final safeguard during extraction: load_in tightly seals the type boundaries
        .load_in::<(i32, String)>()?;

    Ok(())
}