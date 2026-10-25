use crate::render::{capture_one, render_client};
use kova::query::Select;
use models::runtime::comment::Comment;
use models::runtime::order::Order;
use models::runtime::post::Post;
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

#[test]
fn tag_registration_strictly_follows_from_then_join_order() {
    let client = render_client();

    // Even though T2 is referenced before T1 and T0 in `.columns()` and `.filter()`,
    // pre-registration guarantees deterministic alias numbering: FROM(T0 -> 0), JOIN(T1 -> 1), JOIN(T2 -> 2).
    let cap = capture_one(|| {
        client
            .select::<User>()
            .tag::<T0>()
            .inner_join::<Order>(Order::user_id.tag::<T1>().eq(User::id.tag::<T0>()))
            .tag::<T1>()
            .left_join::<Post>(Post::author_id.tag::<T2>().eq(User::id.tag::<T0>()))
            .tag::<T2>()
            .columns((
                Post::title.tag::<T2>(),
                Order::amount.tag::<T1>(),
                User::name.tag::<T0>(),
            ))
            .filter(Post::id.tag::<T2>().gt(10) & User::is_active.tag::<T0>().eq(true))
            .load_in::<(String, f64, String)>()
    });

    assert_eq!(
        cap.sql,
        r#"select "post_2"."title", "order_1"."amount", "user_0"."name" from "user" as "user_0" inner join "order" as "order_1" on "order_1"."user_id" = "user_0"."id" left join "post" as "post_2" on "post_2"."author_id" = "user_0"."id" where "post_2"."id" > ? and "user_0"."is_active" = ?"#
    );
}

#[test]
fn subquery_tag_scope_isolation_and_outer_correlation() {
    let client = render_client();

    // Outer query registers T0(0) and T1(1). Two sibling subqueries reuse T0 locally (getting 2 and 3)
    // while still resolving the correlated outer tag T1(1).
    let cap = capture_one(|| {
        client
            .select::<User>()
            .tag::<T0>()
            .inner_join::<Order>(Order::user_id.tag::<T1>().eq(User::id.tag::<T0>()))
            .tag::<T1>()
            .columns(User::id.tag::<T0>())
            .filter(
                User::id.tag::<T0>().in_select(
                    Select::from::<User>()
                        .tag::<T0>()
                        .columns(User::id.tag::<T0>())
                        .filter(User::id.tag::<T0>().eq(Order::user_id.tag::<T1>())),
                ) & User::id.tag::<T0>().not_in_select(
                    Select::from::<User>()
                        .tag::<T0>()
                        .columns(User::id.tag::<T0>())
                        .filter(User::is_active.tag::<T0>().eq(false)),
                ),
            )
            .load_in::<i32>()
    });

    assert_eq!(
        cap.sql,
        r#"select "user_0"."id" from "user" as "user_0" inner join "order" as "order_1" on "order_1"."user_id" = "user_0"."id" where "user_0"."id" in (select "user_2"."id" from "user" as "user_2" where "user_2"."id" = "order_1"."user_id") and "user_0"."id" not in (select "user_3"."id" from "user" as "user_3" where "user_3"."is_active" = ?)"#
    );
}

#[test]
fn tag_registry_overflow_beyond_8_inline_slots() {
    let client = render_client();

    // 1 FROM + 9 JOINs = 10 distinct tags, crossing `INLINE_TAGS = 8` into `overflow: Vec`.
    let cap = capture_one(|| {
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
            .columns((User::id.tag::<T0>(), Order::id.tag::<T8>(), Comment::id.tag::<T9>()))
            .load_in::<(i32, i32, i32)>()
    });

    assert!(cap.sql.starts_with(
        r#"select "user_0"."id", "order_8"."id", "comment_9"."id" from "user" as "user_0""#
    ));
    assert!(cap.sql.contains(
        r#"inner join "order" as "order_8" on "order_8"."user_id" = "user_7"."id""#
    ));
    assert!(cap.sql.contains(
        r#"inner join "comment" as "comment_9" on "comment_9"."user_id" = "user_0"."id""#
    ));
}