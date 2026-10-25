use crate::render::{capture_one, render_client};
use kova::dsl::func::{
    Count, CumeDist, DenseRank, FrameBound, Lag, Lead, Ntile, PercentRank, Rank, RowNumber, Sum,
    Window,
};
use kova::query::Select;
use models::runtime::comment::Comment;
use models::runtime::order::Order;
use models::runtime::user::User;

struct T0;
struct T1;
struct T2;
struct T3;
struct T4;
struct T5;
struct T6;
struct T7;
struct T8;
struct T9;

#[tokio::test]
async fn pg_exclusive_window_functions_and_frame_bounds() {
    let client = render_client().await;

    let cap = capture_one(
        client
            .select::<Order>()
            .columns((
                RowNumber().over(Window().partition_by(Order::user_id).order_by(Order::id.asc())),
                Rank().over(Window().order_by(Order::amount.desc())),
                DenseRank().over(Window().order_by(Order::amount.desc())),
                PercentRank().over(Window().order_by(Order::amount.asc())),
                CumeDist().over(Window().order_by(Order::amount.asc())),
                Ntile(4).over(Window().partition_by(Order::user_id)),
                Lag(Order::amount).over(Window().order_by(Order::id.asc())),
                Lag(Order::amount)
                    .offset(2)
                    .default(0.0)
                    .over(Window().order_by(Order::id.asc())),
                Lead(Order::amount)
                    .offset(Order::user_id)
                    .default(Order::amount)
                    .over(Window().order_by(Order::id.asc())),
                Sum(Order::amount).over(
                    Window()
                        .partition_by(Order::user_id)
                        .order_by(Order::id.asc())
                        .rows(FrameBound::UnboundedPreceding, Some(FrameBound::CurrentRow)),
                ),
                Count(Order::id).over(
                    Window()
                        .order_by(Order::id.asc())
                        .range(FrameBound::Preceding(1), Some(FrameBound::Following(2))),
                ),
                Sum(Order::amount).over(
                    Window()
                        .order_by(Order::id.asc())
                        .groups(FrameBound::CurrentRow, Some(FrameBound::UnboundedFollowing)),
                ),
            ))
            .load_in::<(i64, i64, i64, f64, f64, i32, Option<f64>, f64, f64, f64, i64, f64)>(),
    )
        .await;

    assert_eq!(
        cap.sql,
        r#"select row_number() over (partition by "order"."user_id" order by "order"."id" asc), rank() over (order by "order"."amount" desc), dense_rank() over (order by "order"."amount" desc), percent_rank() over (order by "order"."amount" asc), cume_dist() over (order by "order"."amount" asc), ntile($1) over (partition by "order"."user_id"), lag("order"."amount") over (order by "order"."id" asc), lag("order"."amount", $2, $3) over (order by "order"."id" asc), lead("order"."amount", "order"."user_id", "order"."amount") over (order by "order"."id" asc), sum("order"."amount") over (partition by "order"."user_id" order by "order"."id" asc rows between unbounded preceding and current row), count("order"."id") over (order by "order"."id" asc range between 1 preceding and 2 following), sum("order"."amount") over (order by "order"."id" asc groups between current row and unbounded following) from "order""#
    );
    assert_eq!(cap.params, "[Int4(4), Int4(2), Float8(0.0)]");
}

#[tokio::test]
async fn pg_tag_registry_scope_isolation_and_overflow() {
    let client = render_client().await;

    let cap = capture_one(
        client
            .select::<User>()
            .tag::<T0>()
            .inner_join::<User>(User::id.tag::<T1>().eq(User::id.tag::<T0>()))
            .tag::<T1>()
            .inner_join::<User>(User::id.tag::<T2>().eq(User::id.tag::<T1>()))
            .tag::<T2>()
            .inner_join::<User>(User::id.tag::<T3>().eq(User::id.tag::<T2>()))
            .tag::<T3>()
            .inner_join::<User>(User::id.tag::<T4>().eq(User::id.tag::<T3>()))
            .tag::<T4>()
            .inner_join::<User>(User::id.tag::<T5>().eq(User::id.tag::<T4>()))
            .tag::<T5>()
            .inner_join::<User>(User::id.tag::<T6>().eq(User::id.tag::<T5>()))
            .tag::<T6>()
            .inner_join::<User>(User::id.tag::<T7>().eq(User::id.tag::<T6>()))
            .tag::<T7>()
            .inner_join::<Order>(Order::user_id.tag::<T8>().eq(User::id.tag::<T7>()))
            .tag::<T8>()
            .inner_join::<Comment>(Comment::user_id.tag::<T9>().eq(User::id.tag::<T0>()))
            .tag::<T9>()
            .filter(
                User::id.tag::<T0>().in_select(
                    Select::from::<User>()
                        .tag::<T0>()
                        .columns(User::id.tag::<T0>())
                        .filter(User::id.tag::<T0>().eq(Order::user_id.tag::<T8>())),
                ),
            )
            .columns((
                Comment::id.tag::<T9>(),
                Order::id.tag::<T8>(),
                User::id.tag::<T0>(),
            ))
            .load_in::<(i32, i32, i32)>(),
    )
        .await;

    assert!(cap.sql.starts_with(
        r#"select "comment_9"."id", "order_8"."id", "user_0"."id" from "user" as "user_0""#
    ));
    // Subquery reuses T0 locally as `user_10` while referencing outer overflow slot `order_8`.
    assert!(cap.sql.ends_with(
        r#"where "user_0"."id" in (select "user_10"."id" from "user" as "user_10" where "user_10"."id" = "order_8"."user_id")"#
    ));
}