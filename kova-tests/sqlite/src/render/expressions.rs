use crate::render::{capture_one, render_client};
use kova::dsl::func::{
    Abs, Avg, Count, FrameBound, GroupConcat, Length, Lower, Max, Sum, Upper, Window,
};
use kova::query::Select;
use models::runtime::order::Order;
use models::runtime::user::User;

#[test]
fn boolean_condition_precedence_and_flattening() {
    let client = render_client();

    // Consecutive `&` flatten into a single `ConditionNode::And` Vec, nested `|` are parenthesized,
    // and `!` wraps its inner tree in `not (...)`.
    let cap = capture_one(|| {
        client
            .select::<User>()
            .filter(
                User::id.eq(1)
                    & User::is_active.eq(true)
                    & (User::name.eq("a")
                    | User::name.eq("b")
                    | (User::balance.gt(0.0) & User::balance.lt(100.0)))
                    & !(User::bio.is_null() | User::name.like("test%")),
            )
            .load()
    });

    assert_eq!(
        cap.sql,
        r#"select * from "user" where "user"."id" = ? and "user"."is_active" = ? and ("user"."name" = ? or "user"."name" = ? or ("user"."balance" > ? and "user"."balance" < ?)) and not ("user"."bio" is null or "user"."name" like ?)"#
    );
    assert_eq!(
        cap.params,
        r#"[Int8(1), Int8(1), Text("a"), Text("b"), Float8(0.0), Float8(100.0), Text("test%")]"#
    );
}

#[test]
fn empty_in_list_and_render_bare_subselect() {
    let client = render_client();

    let cap = capture_one(|| {
        client
            .select::<User>()
            .filter(
                User::id.in_list(Vec::<i32>::new())
                    | User::id.not_in_list(Vec::<i32>::new())
                    | User::id.in_select(Select::from::<Order>().columns(Order::user_id)),
            )
            .load()
    });

    // Empty `in_list` -> `false`, empty `not_in_list` -> `true`,
    // `in_select` uses `render_bare` to avoid double parentheses `in ((select ...))`.
    assert_eq!(
        cap.sql,
        r#"select * from "user" where false or true or "user"."id" in (select "order"."user_id" from "order")"#
    );
    assert_eq!(cap.params, "[]");
}

#[test]
fn math_nesting_and_agg_modifiers_isolation() {
    let client = render_client();

    // Verifies that `ctx.agg_modifiers.take()` consumes `distinct` and `order_by` before rendering
    // inner expressions (`Lower(Upper(...))` / `Abs(...)`), preventing modifier leakage.
    let cap = capture_one(|| {
        client
            .select::<User>()
            .inner_join::<Order>(Order::user_id.eq(User::id))
            .columns((
                User::balance.add(10.0).sub(2.0).mul(3.0).div(4.0).alias(User::balance),
                Count(Lower(Upper(User::name)))
                    .distinct()
                    .order_by(User::id.desc().nulls_first())
                    .filter(User::is_active.eq(true)),
                GroupConcat(User::name)
                    .separator(" | ")
                    .order_by(User::name.asc())
                    .filter(Order::amount.gt(0.0)),
                Max(Abs(Order::amount)).over(
                    Window()
                        .partition_by((Order::user_id, Order::is_active))
                        .order_by((Order::id.asc(), Order::is_active.eq(true)))
                        .rows(FrameBound::UnboundedPreceding, Some(FrameBound::CurrentRow)),
                ),
                Avg(Order::amount).over(
                    Window()
                        .order_by(Order::id.asc())
                        .range(FrameBound::Preceding(2), Some(FrameBound::Following(3))),
                ),
                Sum(Order::amount).over(
                    Window()
                        .partition_by(Order::user_id)
                        .groups(FrameBound::CurrentRow, None),
                ),
                Length(User::name),
            ))
            .group_by((User::id, Order::id))
            .load_in::<(f64, i64, Option<String>, Option<f64>, Option<f64>, Option<f64>, i64)>()
    });

    assert_eq!(
        cap.sql,
        r#"select (((("user"."balance" + ?) - ?) * ?) / ?) as "balance", count(distinct lower(upper("user"."name")) order by "user"."id" desc nulls first) filter (where "user"."is_active" = ?), group_concat("user"."name", ' | ' order by "user"."name" asc) filter (where "order"."amount" > ?), max(abs("order"."amount")) over (partition by "order"."user_id", "order"."is_active" order by "order"."id" asc, "order"."is_active" = ? asc rows between unbounded preceding and current row), avg("order"."amount") over (order by "order"."id" asc range between 2 preceding and 3 following), sum("order"."amount") over (partition by "order"."user_id" groups current row), length("user"."name") from "user" inner join "order" on "order"."user_id" = "user"."id" group by "user"."id", "order"."id""#
    );
    assert_eq!(
        cap.params,
        "[Float8(10.0), Float8(2.0), Float8(3.0), Float8(4.0), Int8(1), Float8(0.0), Int8(1)]"
    );
}