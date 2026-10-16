#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::models::comment::Comment;
use crate::models::order::Order;
use crate::models::post::Post;
use crate::models::user::User;
use kova::Options;
use kova::dsl::Column;
use kova::dsl::func::Count;
use kova::query::Select;
use kova::types::IdentifierMeta;

struct T1;
struct T2;
struct T3;
struct CTEAlias;

async fn compile_check_tags() -> Result<()> {
    let client = Options::from("").build()?;

    // 1. Self Join with Tags
    let _ = client
        .select::<User>()
        .tag::<T1>()
        .inner_join::<User>(User::id.tag::<T1>().eq(User::id.tag::<T2>()))
        .tag::<T2>()
        .columns((User::id.tag::<T1>(), User::name.tag::<T2>()))
        .filter(User::is_active.tag::<T1>().eq(true))
        .load_in::<(i32, String)>()
        .await?;

    // 2. Virtual/Identifier Column Tagging
    #[allow(non_upper_case_globals)]
    const virtual_col: Column<f64, IdentifierMeta> =
        Column::identifier("virtual_table", "score");

    let _ = client
        .select::<User>()
        .columns((User::id, virtual_col.tag::<CTEAlias>()))
        .filter(virtual_col.tag::<CTEAlias>().gt(50.0_f64))
        .load_in::<(i32, f64)>()
        .await?;

    // 3. Isolated Tagging Scope Inside Subqueries
    let _ = client
        .select::<Order>()
        .filter(
            Order::user_id.in_select(
                Select::from::<User>()
                    .tag::<T1>()
                    .columns(User::id.tag::<T1>())
                    .filter(User::is_active.tag::<T1>().eq(true)),
            ),
        )
        .load_as::<Order>()
        .await?;

    // 4. Deep Chained Joins with Multiple Tags
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
            Count(Comment::id.tag::<T3>()).alias(User::id),
        ))
        .group_by((User::name.tag::<T1>(), Post::title.tag::<T2>()))
        .load_in::<(String, String, i64)>()
        .await?;

    Ok(())
}