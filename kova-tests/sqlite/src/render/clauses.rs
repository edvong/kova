use crate::render::{capture_all, capture_one, render_client};
use kova::dsl::func::{Count, Lower, Sum, Wildcard};
use kova::query::Select;
use models::runtime::comment::Comment;
use models::runtime::order::Order;
use models::runtime::post::Post;
use models::runtime::top_spender::TopSpender;
use models::runtime::user::User;

#[test]
fn select_clause_overwrite_semantics_and_condition_ordering() {
    let client = render_client();

    // Verifies that repeated calls to columns / filter / group_by / having / order_by / limit / offset
    // overwrite previous state without leaking discarded bind parameters, and tests Condition in ORDER BY.
    let cap = capture_one(|| {
        client
            .select::<User>()
            .columns((User::id, User::name))
            .columns(User::balance)
            .filter(User::name.eq("discarded_param"))
            .filter(User::is_active.eq(true))
            .group_by(User::name)
            .group_by(User::id)
            .having(Count(Wildcard).gt(999_i64))
            .having(Sum(User::balance).gt(100.0))
            .order_by(User::name.asc())
            .order_by((User::id.desc().nulls_last(), User::is_active.eq(true)))
            .limit(10)
            .limit(25)
            .offset(5)
            .offset(50)
            .load_in::<f64>()
    });

    assert_eq!(
        cap.sql,
        r#"select "user"."balance" from "user" where "user"."is_active" = ? group by "user"."id" having sum("user"."balance") > ? order by "user"."id" desc nulls last, "user"."is_active" = ? asc limit 25 offset 50"#
    );
    assert_eq!(cap.params, "[Int8(1), Float8(100.0), Int8(1)]");
}

#[test]
fn sqlite_pagination_offset_without_limit_renders_limit_minus_one() {
    let client = render_client();

    let cap = capture_one(|| client.select::<User>().offset(15).load());
    assert_eq!(cap.sql, r#"select * from "user" limit -1 offset 15"#);
    assert_eq!(cap.params, "[]");
}

#[test]
fn join_variants_on_using_and_cross() {
    let client = render_client();

    let cap = capture_one(|| {
        client
            .select::<Order>()
            .distinct()
            .columns(Order::id)
            .right_join::<Comment>((Comment::id, Comment::user_id))
            .left_join::<Post>(Post::content)
            .inner_join::<User>(Order::user_id.eq(User::id) & User::is_active.eq(true))
            .full_join::<Post>(Order::id.eq(User::id))
            .cross_join::<Comment>()
            .load_in::<i32>()
    });

    assert_eq!(
        cap.sql,
        r#"select distinct "order"."id" from "order" right join "comment" using ("id", "user_id") left join "post" using ("content") inner join "user" on "order"."user_id" = "user"."id" and "user"."is_active" = ? full join "post" on "order"."id" = "user"."id" cross join "comment""#
    );
    assert_eq!(cap.params, "[Int8(1)]");
}

#[test]
fn cte_with_multiple_nodes_and_subquery_projection() {
    let client = render_client();

    let cap = capture_one(|| {
        client
            .select::<User>()
            .with::<TopSpender>(
                Select::from::<Order>()
                    .columns((
                        Order::user_id,
                        Sum(Order::amount).alias(TopSpender::total_amount),
                        Count(Wildcard).alias(TopSpender::order_count),
                    ))
                    .group_by(Order::user_id)
                    .having(Sum(Order::amount).gt(50.0)),
            )
            .inner_join::<TopSpender>(User::id.eq(TopSpender::user_id))
            .columns((
                User::id,
                TopSpender::total_amount,
                Select::from::<Comment>()
                    .columns(Count(Wildcard))
                    .filter(Comment::user_id.eq(User::id))
                    .alias(TopSpender::order_count),
            ))
            .load_in::<(i32, f64, i64)>()
    });

    assert_eq!(
        cap.sql,
        r#"with "top_spender" as (select "order"."user_id", sum("order"."amount") as "total_amount", count(*) as "order_count" from "order" group by "order"."user_id" having sum("order"."amount") > ?) select "user"."id", "top_spender"."total_amount", (select count(*) from "comment" where "comment"."user_id" = "user"."id") as "order_count" from "user" inner join "top_spender" on "user"."id" = "top_spender"."user_id""#
    );
    assert_eq!(cap.params, "[Float8(50.0)]");
}

#[test]
fn insert_sources_owned_borrowed_slice_empty_and_insert_select() {
    let client = render_client();

    // 1. Empty Vec insert returns Ok(0) immediately without emitting any SQL.
    let (res, logs) = capture_all(|| client.insert(Vec::<User>::new()).execute());
    assert_eq!(res.unwrap(), 0);
    assert!(logs.is_empty());

    // 2. Borrowed slice `&[User]` insert + Returning (verifying `#[kova(auto)]` skips `id`).
    let users = [
        User { id: 99, name: "alice".into(), bio: None, balance: 10.0, is_active: true },
        User {
            id: 100,
            name: "bob".into(),
            bio: Some("dev".into()),
            balance: 20.0,
            is_active: false,
        },
        User { id: 101, name: "charlie".into(), bio: None, balance: 30.0, is_active: true },
    ];
    let slice: &[User] = &users[0..2];

    let cap = capture_one(|| {
        client
            .insert(slice)
            .returning((User::id, Lower(User::name).alias(User::name)))
            .execute_in::<(i32, String)>()
    });
    assert_eq!(
        cap.sql,
        r#"insert into "user" ("name", "bio", "balance", "is_active") values (?, ?, ?, ?), (?, ?, ?, ?) returning "user"."id", lower("user"."name") as "name""#
    );
    assert_eq!(
        cap.params,
        r#"[Text("alice"), Null, Float8(10.0), Int8(1), Text("bob"), Text("dev"), Float8(20.0), Int8(0)]"#
    );

    // 3. INSERT ... SELECT
    let cap_sel = capture_one(|| {
        render_client()
            .insert::<User>(
                Select::from::<User>()
                    .columns((User::name, User::bio, User::balance, User::is_active))
                    .filter(User::is_active.eq(true)),
            )
            .execute()
    });
    assert_eq!(
        cap_sel.sql,
        r#"insert into "user" ("name", "bio", "balance", "is_active") select "user"."name", "user"."bio", "user"."balance", "user"."is_active" from "user" where "user"."is_active" = ?"#
    );
}

#[test]
fn update_and_delete_overwrite_and_returning() {
    let client = render_client();

    let cap_upd = capture_one(|| {
        client
            .update::<Order>()
            .set(Order::amount.set(1.0))
            .set((
                Order::amount.increment(15.5),
                Order::user_id.decrement(1),
                Order::is_active.set(false),
            ))
            .from::<Post>()
            .from::<(User, Comment)>()
            .filter(Order::id.eq(999))
            .filter(Order::user_id.eq(User::id) & User::id.eq(Comment::user_id))
            .returning(Order::id)
            .execute_in::<i32>()
    });

    assert_eq!(
        cap_upd.sql,
        r#"update "order" set "amount" = "order"."amount" + ?, "user_id" = "order"."user_id" - ?, "is_active" = ? from "user", "comment" where "order"."user_id" = "user"."id" and "user"."id" = "comment"."user_id" returning "order"."id""#
    );
    assert_eq!(cap_upd.params, "[Float8(15.5), Int8(1), Int8(0)]");

    let cap_del = capture_one(|| {
        client
            .delete::<User>()
            .filter(User::id.eq(1))
            .filter(User::id.eq(2))
            .returning((User::id, User::name))
            .execute_in::<(i32, String)>()
    });
    assert_eq!(
        cap_del.sql,
        r#"delete from "user" where "user"."id" = ? returning "user"."id", "user"."name""#
    );
    assert_eq!(cap_del.params, "[Int8(2)]");
}