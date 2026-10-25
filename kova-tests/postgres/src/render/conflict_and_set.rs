use crate::render::{capture_one, render_client};
use kova::dsl::Constraint;
use kova::dsl::func::Lower;
use kova::query::Select;
use models::runtime::user::User;

#[tokio::test]
async fn pg_on_conflict_columns_expressions_and_named_constraint() {
    let client = render_client().await;
    let u = User {
        id: 0,
        name: "alice".into(),
        bio: None,
        balance: 100.0,
        is_active: true,
    };

    // 1. ON CONFLICT ON CONSTRAINT "..." DO NOTHING
    let cap_constraint = capture_one(
        client
            .insert(&u)
            .on_conflict(Constraint("uk_user_email_idx"))
            .do_nothing()
            .execute(),
    )
        .await;

    assert_eq!(
        cap_constraint.sql,
        r#"insert into "user" ("name", "bio", "balance", "is_active") values ($1, $2, $3, $4) on conflict on constraint "uk_user_email_idx" do nothing"#
    );

    client.delete::<User>().execute().await.unwrap();

    // 2. ON CONFLICT ("id", lower("name")) DO UPDATE SET ... WHERE ...
    // Verifies `unqualify_columns` strips table prefixes in conflict targets and restores them in DO UPDATE / WHERE.
    let cap_upsert = capture_one(
        client
            .insert(&u)
            .on_conflict((User::id, Lower(User::name)))
            .update((User::name.excluded(), User::balance.increment(10.0)))
            .filter(User::balance.gt(0.0) | User::is_active.eq(false))
            .returning(User::id)
            .execute_in::<i32>(),
    )
        .await;

    assert_eq!(
        cap_upsert.sql,
        r#"insert into "user" ("name", "bio", "balance", "is_active") values ($1, $2, $3, $4) on conflict ("id", lower("name")) do update set "name" = excluded."name", "balance" = "user"."balance" + $5 where "user"."balance" > $6 or "user"."is_active" = $7 returning "user"."id""#
    );
    assert_eq!(
        cap_upsert.params,
        r#"[Text("alice"), Null, Float8(100.0), Bool(true), Float8(10.0), Float8(0.0), Bool(false)]"#
    );
}

#[tokio::test]
async fn pg_set_operations_unqualify_trailing_order_by() {
    let client = render_client().await;

    let cap = capture_one(
        client
            .select::<User>()
            .columns((User::id, User::name))
            .filter(User::is_active.eq(true))
            .union(
                Select::from::<User>()
                    .columns((User::id, User::name))
                    .filter(User::balance.gt(50.0)),
            )
            .union_all(Select::from::<User>().columns((User::id, User::name)))
            .intersect(Select::from::<User>().columns((User::id, User::name)))
            .except(
                Select::from::<User>()
                    .columns((User::id, User::name))
                    .filter(User::name.ilike("%test%")),
            )
            .order_by((User::id.desc(), User::name.asc()))
            .limit(5)
            .offset(10)
            .load_in::<(i32, String)>(),
    )
        .await;

    assert_eq!(
        cap.sql,
        r#"select "user"."id", "user"."name" from "user" where "user"."is_active" = $1 union select "user"."id", "user"."name" from "user" where "user"."balance" > $2 union all select "user"."id", "user"."name" from "user" intersect select "user"."id", "user"."name" from "user" except select "user"."id", "user"."name" from "user" where "user"."name" ilike $3 order by "id" desc, "name" asc limit 5 offset 10"#
    );
    assert_eq!(cap.params, r#"[Bool(true), Float8(50.0), Text("%test%")]"#);
}