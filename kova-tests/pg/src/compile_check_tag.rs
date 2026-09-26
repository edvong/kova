#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::models::comment::Comment;
use crate::models::order::Order;
use crate::models::post::Post;
use crate::models::user::User;
use kova::Options;
use kova::dsl::Identifier;
use kova::dsl::func::Count;
use kova::query::Select;

// Marker types for compile-time static tagging
struct T1;
struct T2;
struct T3;
struct CTEAlias;

/// Stress test for the TAG mechanism and Type-State Machine transitions in Self Joins.
async fn compile_check_tags() -> Result<()> {
    let client = Options::from("").build()?;

    // ==========================================
    // Scenario 1: Basic Self Join
    // Validates:
    // 1. Select::tag() transition from `JustAddFrom` -> `Normal`
    // 2. Select::tag() transition from `JustAddJoin` -> `Normal`
    // 3. Column::tag() overriding the column's source table identifier
    // ==========================================
    let _ = client
        .select::<User>()
        .tag::<T1>() // Tag the primary User table as T1
        .inner_join::<User>(
            // Perfectly bypasses same-name column conflicts: ON T1.id = T2.id
            User::id.tag::<T1>().eq(User::id.tag::<T2>()),
        )
        .tag::<T2>() // Tag the joined User table as T2
        .columns((
            User::id.tag::<T1>(),
            User::name.tag::<T2>(),
        ))
        .filter(User::is_active.tag::<T1>().eq(true))
        .load_in::<(i32, String)>()
        .await?;

    // ==========================================
    // Scenario 2: Identifier Tagging (Virtual Columns / CTEs)
    // Validates: Dynamic/Virtual columns correctly inherit TAG assignments
    // ==========================================
    #[allow(non_upper_case_globals)]
    const virtual_col:  Identifier<f64> = Identifier::<f64>::new("virtual_table", "score");

    let _ = client
        .select::<User>()
        .columns((
            User::id,
            virtual_col.tag::<CTEAlias>(), // Bind alias at projection phase
        ))
        .filter(virtual_col.tag::<CTEAlias>().gt(50.0_f64))
        .load_in::<(i32, f64)>()
        .await?;

    // ==========================================
    // Scenario 3: Isolated Tagging Scope Inside Subqueries
    // Validates: Select::from().tag() accurately handles state transitions
    // without leaking outer scope context.
    // ==========================================
    let _ = client
        .select::<Order>()
        .filter(
            Order::user_id.in_select(
                Select::from::<User>()
                    .tag::<T1>() // The FROM clause within the subquery intercepts the tag successfully
                    .columns(User::id.tag::<T1>())
                    .filter(User::is_active.tag::<T1>().eq(true)),
            ),
        )
        .load_as::<Order>()
        .await?;

    // ==========================================
    // Scenario 4: Deep Chained Joins with Multiple Tags
    // Validates: Continuous flow of (Join -> JustAddJoin -> Tag -> Normal) state cycle
    // and application of tags inside aggregate functions.
    // ==========================================
    let _ = client
        .select::<User>()
        .tag::<T1>()
        .left_join::<Post>(Post::author_id.tag::<T2>().eq(User::id.tag::<T1>()))
        .tag::<T2>()
        .right_join::<Comment>(Comment::post_id.tag::<T3>().eq(Post::id.tag::<T2>()))
        .tag::<T3>()
        .columns((
            User::name.tag::<T1>(),
            Post::title.tag::<T2>(),
            Count(Comment::id.tag::<T3>()).alias(User::id), // Tag applied inside an aggregate wrapper
        ))
        .group_by((User::name.tag::<T1>(), Post::title.tag::<T2>()))
        .load_in::<(String, String, i64)>()
        .await?;

    Ok(())
}