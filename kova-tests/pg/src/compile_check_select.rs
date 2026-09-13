#![allow(unreachable_code, dead_code, unused)]

use crate::Result;
use crate::models::comment::Comment;
use crate::models::order::Order;
use crate::models::post::Post;
use crate::models::user::User;
use crate::view::post_view::{CommentDto, PostView};
use kova::Options;
use kova::dsl::func::{
    Abs, Avg, CountAll, JsonbAgg, JsonbObject, Length, Lower, Max, Min, Sum, Upper,
};
use kova::dsl::func::{ArrayAgg, Count};
use kova::dsl::{LockMode, Subselect, WaitPolicy};

fn compile_check_select() {
    let client = Options::from("").tls(|| unimplemented!()).build().unwrap();

    client
        .select::<User>()
        // Stress test IntoArgs<Projection> polymorphic tuple: standard column + aggregate function + alias extension + constant
        .columns(CountAll)
        .columns(User::name)
        .columns(Sum(Order::amount))
        .columns((
            User::id,
            Sum(Order::amount),
            Count(User::id).alias(User::id),
        ))
        .columns((
            User::id,
            Order::amount,
            Count(User::id).alias(User::id), // Core function wrapper
            Sum(Order::amount).alias(User::name),
            Min(Order::amount),
            Max(Order::amount),
            Lower(User::name),
            Upper(User::name),
            Abs(Order::amount),
            Length(User::name),
            ArrayAgg(Order::amount)
                .distinct()
                .order_by(Order::id.desc().nulls_first())
                .filter(Order::is_active.eq(true) & Order::amount.gt(100_f64))
                .alias(User::name), // Forcefully borrow alias to verify type boundaries
        ))
        // 2. Stress test JOIN boundaries: generic Model parameters + Column vs Column comparison
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .left_join::<Order>(Order::is_active.ne(false)) // Column vs Value
        // 3. Stress test BitAnd (&) / BitOr (|) nesting and all condition operators
        .filter(
            User::name.like("%admin%")
                & (Order::amount.between(10.5_f64, 99.9_f64)
                    | Order::amount.not_between(0.0_f64, 1.0_f64)
                    | Order::id.in_list(vec![1, 2, 3])
                    | Order::id.not_in(vec![4, 5, 6])
                    | Order::amount.ge(0.0_f64)
                    | Order::amount.le(1000.0_f64)
                    | Order::raw_data.is_not_null()),
        )
        // 4. Stress test Group By polymorphic tuple: mixed grouping of columns + functions
        .group_by(User::id)
        .group_by((User::id, Order::is_active))
        // 5. Stress test Having boundaries: using extension function as left-hand side (LHS) operand
        .having(Avg(User::id).eq(100))
        .having(ArrayAgg(Order::id).gt(100_i32))
        // 6. Stress test Order By polymorphic tuple: explicit modifiers + implicit derivation
        .order_by(User::name.asc())
        .order_by((
            Order::amount.desc().nulls_last(),
            User::name.asc(),
            User::id, // Implicitly converted to OrderOperand without modifiers
        ))
        // 7. Stress test Pagination extensions
        .limit(50)
        .offset(100)
        // 8. Stress test Locking extensions
        .lock::<User>(LockMode::NoKeyUpdate, Some(WaitPolicy::NoWait))
        .lock_all(LockMode::Share, None);
}

/// Stress Test 2: Lifecycle of Load API and extraction limits of macro-generated Tuples
async fn compile_check_pg_load_endpoints() -> Result<()> {
    let client = Options::from("").build()?;

    // Endpoint 1: Load main model
    let _users = client.select::<User>().load().await?;

    // Endpoint 2: Map to another model (Verify U: FromRow)
    let _orders = client.select::<Order>().load().await?;
    let _orders = client.select::<User>().load_as::<(User, Order)>().await?;

    // Endpoint 3: Extract scalar (Single column, triggers basic implementation of FromColumn)
    let _ids = client.select::<User>().load_in::<Option<f64>>().await?;

    // Endpoint 4: Extract single column Option
    let _optional_amounts = client.select::<User>().load_in::<Option<f64>>().await?;

    // Endpoint 5: Extract giant strongly-typed Tuple
    let _complex_rows = client
        .select::<User>()
        .load_in::<(i32, String, Option<f64>, bool, Vec<u8>)>()
        .await?;

    Ok(())
}


async fn compile_check_complex_jsonb_subquery() -> Result<()> {
    let client = Options::from("").build()?;

    let _views = client
        .select::<Post>()
        .columns((
            Post::id,
            Post::title,
            Post::content,
            JsonbObject((User::id, User::name)).alias(PostView::author),
            Subselect::<Comment>::columns(
                JsonbAgg(
                    JsonbObject((
                        Comment::id,
                        Comment::content,
                        JsonbObject((User::id, User::name)).alias(CommentDto::commenter),
                    ))
                )
            )
            .inner_join::<User>(Comment::user_id.eq(User::id))
            .filter(Comment::post_id.eq(Post::id))
            .alias(PostView::comments),
        ))
        .inner_join::<User>(Post::author_id.eq(User::id))
        .load_as::<PostView>()
        .await?;

    Ok(())
}
