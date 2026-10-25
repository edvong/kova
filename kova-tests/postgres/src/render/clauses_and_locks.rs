use crate::render::{capture_all, capture_one, render_client};
use kova::dsl::{LockMode, WaitPolicy};
use kova::dsl::func::{Count, Lower, Sum, Wildcard};
use kova::query::Select;
use models::runtime::comment::Comment;
use models::runtime::order::Order;
use models::runtime::post::Post;
use models::runtime::user::User;

#[tokio::test]
async fn select_clause_overwrite_condition_ordering_and_numbered_placeholders() {
    let client = render_client().await;

    // Verifies clause overwrite semantics, Condition in ORDER BY, and sequential $1..$N numbering.
    let cap = capture_one(
        client
            .select::<User>()
            .columns(User::id)
            .columns((User::name, User::balance))
            .filter(User::name.eq("discarded"))
            .filter(User::is_active.eq(true) & User::balance.ge(10.0))
            .group_by(User::id)
            .group_by((User::name, User::balance, User::is_active))
            .having(Count(Wildcard).gt(100_i64))
            .having(Sum(User::balance).gt(500.0))
            .order_by(User::id.asc())
            .order_by((
                User::balance.desc().nulls_last(),
                User::name.asc().nulls_first(),
                User::is_active.eq(true),
            ))
            .limit(1)
            .limit(20)
            .offset(2)
            .offset(40)
            .load_in::<(String, f64)>(),
    )
        .await;

    assert_eq!(
        cap.sql,
        r#"select "user"."name", "user"."balance" from "user" where "user"."is_active" = $1 and "user"."balance" >= $2 group by "user"."name", "user"."balance", "user"."is_active" having sum("user"."balance") > $3 order by "user"."balance" desc nulls last, "user"."name" asc nulls first, "user"."is_active" = $4 asc limit 20 offset 40"#
    );
    assert_eq!(
        cap.params,
        "[Bool(true), Float8(10.0), Float8(500.0), Bool(true)]"
    );
}

#[tokio::test]
async fn pg_row_locking_modes_and_wait_policies() {
    let client = render_client().await;

    let cap = capture_one(
        client
            .select::<User>()
            .inner_join::<Order>(Order::user_id.eq(User::id))
            .filter(User::id.eq(1))
            .lock::<User>(LockMode::Update, Some(WaitPolicy::NoWait))
            .lock::<Order>(LockMode::NoKeyUpdate, Some(WaitPolicy::SkipLocked))
            .lock_all(LockMode::KeyShare, None)
            .load(),
    )
        .await;

    assert_eq!(
        cap.sql,
        r#"select * from "user" inner join "order" on "order"."user_id" = "user"."id" where "user"."id" = $1 for update of "user" nowait for no key update of "order" skip locked for key share"#
    );
    assert_eq!(cap.params, "[Int4(1)]");
}

#[tokio::test]
async fn pg_polymorphic_joins_borrowed_slice_insert_and_mutation_overwrites() {
    let client = render_client().await;

    // 1. Empty Vec insert returns Ok(0) immediately without emitting SQL.
    let (res, logs) = capture_all(client.insert(Vec::<User>::new()).execute()).await;
    assert_eq!(res.unwrap(), 0);
    assert!(logs.is_empty());

    // 2. Borrowed slice `&[User]` insert + Returning.
    let batch = [
        User { id: 0, name: "u1".into(), bio: None, balance: 1.0, is_active: true },
        User { id: 0, name: "u2".into(), bio: Some("b".into()), balance: 2.0, is_active: false },
    ];
    let cap_slice = capture_one(
        client
            .insert(&batch[..])
            .returning(User::id)
            .execute_in::<i32>(),
    )
        .await;
    assert_eq!(
        cap_slice.sql,
        r#"insert into "user" ("name", "bio", "balance", "is_active") values ($1, $2, $3, $4), ($5, $6, $7, $8) returning "user"."id""#
    );

    // 3. JOIN variants (ON, USING single column, USING column tuple, CROSS JOIN).
    struct U2;
    struct P2;

    let cap_join = capture_one(
        client
            .select::<Order>()
            .columns(Order::id)
            .left_join::<Post>(Post::id)
            .right_join::<Comment>((Comment::id, Comment::user_id))
            .inner_join::<User>(Order::user_id.eq(User::id))
            .full_join::<User>(Order::user_id.eq(User::id.tag::<U2>()))
            .tag::<U2>()
            .cross_join::<Post>()
            .tag::<P2>()
            .load_in::<i32>(),
    )
        .await;

    assert_eq!(
        cap_join.sql,
        r#"select "order"."id" from "order" left join "post" using ("id") right join "comment" using ("id", "user_id") inner join "user" on "order"."user_id" = "user"."id" full join "user" as "user_0" on "order"."user_id" = "user_0"."id" cross join "post" as "post_1""#
    );

    // 4. Update overwrite semantics and Returning.
    let cap_upd = capture_one(
        client
            .update::<User>()
            .set(User::name.set("ignored"))
            .set((User::balance.increment(25.0), User::is_active.set(false)))
            .from::<Post>()
            .from::<(Order, Comment)>()
            .filter(User::id.eq(1))
            .filter(User::id.eq(Order::user_id) & Order::id.eq(Comment::id))
            .returning((User::id, Lower(User::name)))
            .execute_in::<(i32, String)>(),
    )
        .await;

    assert_eq!(
        cap_upd.sql,
        r#"update "user" set "balance" = "user"."balance" + $1, "is_active" = $2 from "order", "comment" where "user"."id" = "order"."user_id" and "order"."id" = "comment"."id" returning "user"."id", lower("user"."name")"#
    );
    assert_eq!(cap_upd.params, "[Float8(25.0), Bool(false)]");

    // 5. INSERT ... SELECT
    let cap_ins_sel = capture_one(
        client
            .insert::<Post>(
                Select::from::<User>()
                    .columns((User::name, User::name, User::id))
                    .filter(User::is_active.eq(true)),
            )
            .returning(Post::id)
            .execute_in::<i32>(),
    )
        .await;

    assert_eq!(
        cap_ins_sel.sql,
        r#"insert into "post" ("title", "content", "author_id") select "user"."name", "user"."name", "user"."id" from "user" where "user"."is_active" = $1 returning "post"."id""#
    );
    assert_eq!(cap_ins_sel.params, "[Bool(true)]");
}