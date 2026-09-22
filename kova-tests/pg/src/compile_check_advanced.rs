#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::codec::models::EventLog;
use crate::models::comment::Comment;
use crate::models::order::Order;
use crate::models::post::Post;
use crate::models::user::User;
use kova::Options;

/// Test 1: Polymorphic Joins (Mixing ON Condition and USING Column/Tuple inference)
async fn compile_check_advanced_joins() -> Result<()> {
    let client = Options::from("").build()?;

    let _ = client
        .select::<Order>()
        // Scenario A: ON mode, passing in a fully built boolean condition tree.
        .inner_join::<User>(Order::user_id.eq(User::id) & User::is_active.eq(true))
        // Scenario B: USING mode, passing a single column identifier.
        .left_join::<Post>(Post::author_id)
        // Scenario C: USING mode with a heterogeneous column tuple (stress-testing the internal macro inference boundaries).
        .right_join::<Comment>((Comment::user_id, Comment::post_id))
        // Scenario D: Full Join stitched together with complex function nodes.
        .full_join::<EventLog>(EventLog::description.like("%error%") & EventLog::id.is_not_null())
        // Scenario E: Cross Join, zero parameters, entirely unconditional.
        .cross_join::<User>()
        .load_as::<Order>()
        .await?;

    Ok(())
}

/// Test 2: Upsert Where (Secondary condition filtering during conflict updates)
async fn compile_check_upsert_where() -> Result<()> {
    let client = Options::from("").build()?;
    let users = vec![User::default(), User::default()];

    client
        .insert(&users)
        .on_conflict((User::id, User::name))
        .update((
            User::name.set("conflict_resolved"),
            User::is_active.set(true),
        ))
        // Strict type validation here: making sure `filter` is actually accessible in the Conflicted state and accepts a standard Condition.
        .filter(User::balance.gt(0.0_f64) | User::is_active.eq(false))
        .execute()
        .await?;

    Ok(())
}

/// Test 3: Update From (Cross-table correlated updates)
async fn compile_check_update_from() -> Result<()> {
    let client = Options::from("").build()?;

    client
        .update::<Order>()
        .set((
            Order::amount.set(999.9_f64),
            Order::is_active.set(true),
        ))
        // Tacking on additional data sources to verify the state machine chains smoothly.
        .from::<User>()
        .from::<(User, Post)>()
        .filter(
            // Cross-table filtering constraint: proving that the WHERE clause can safely reference fields from external tables dragged in by FROM.
            Order::user_id.eq(User::id)
                & User::id.eq(Post::author_id)
                & User::is_active.eq(true)
        )
        .execute()
        .await?;

    Ok(())
}