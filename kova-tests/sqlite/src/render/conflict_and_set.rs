use crate::render::{capture_one, render_client};
use kova::dsl::func::Lower;
use kova::query::Select;
use models::runtime::user::User;

#[test]
fn on_conflict_unqualifies_targets_and_restores_in_do_update_where() {
    let client = render_client();
    let user = User {
        id: 1,
        name: "alice".into(),
        bio: None,
        balance: 100.0,
        is_active: true,
    };

    // Verifies that ON CONFLICT targets strip the `"user".` table prefix (`unqualify_columns = true`),
    // while DO UPDATE SET and WHERE immediately restore full `"user"."balance"` qualification.
    let cap = capture_one(|| {
        client
            .insert(&user)
            .on_conflict((User::is_active, Lower(User::name)))
            .update((User::name.excluded(), User::balance.increment(50.0)))
            .filter(User::balance.gt(10.0) & User::is_active.eq(true))
            .execute()
    });

    assert_eq!(
        cap.sql,
        r#"insert into "user" ("name", "bio", "balance", "is_active") values (?, ?, ?, ?) on conflict ("is_active", lower("name")) do update set "name" = excluded."name", "balance" = "user"."balance" + ? where "user"."balance" > ? and "user"."is_active" = ?"#
    );
    assert_eq!(
        cap.params,
        r#"[Text("alice"), Null, Float8(100.0), Int8(1), Float8(50.0), Float8(10.0), Int8(1)]"#
    );
}

#[test]
fn set_operations_unqualify_trailing_order_by() {
    let client = render_client();

    let cap = capture_one(|| {
        client
            .select::<User>()
            .columns((User::id, User::name))
            .filter(User::is_active.eq(true))
            .union(
                Select::from::<User>()
                    .columns((User::id, User::name))
                    .filter(User::balance.gt(100.0)),
            )
            .union_all(Select::from::<User>().columns((User::id, User::name)))
            .intersect(Select::from::<User>().columns((User::id, User::name)))
            .except(
                Select::from::<User>()
                    .columns((User::id, User::name))
                    .filter(User::name.like("%bot%")),
            )
            .order_by((User::id.desc(), User::name.asc()))
            .limit(10)
            .load_in::<(i32, String)>()
    });

    assert_eq!(
        cap.sql,
        r#"select "user"."id", "user"."name" from "user" where "user"."is_active" = ? union select "user"."id", "user"."name" from "user" where "user"."balance" > ? union all select "user"."id", "user"."name" from "user" intersect select "user"."id", "user"."name" from "user" except select "user"."id", "user"."name" from "user" where "user"."name" like ? order by "id" desc, "name" asc limit 10"#
    );
    assert_eq!(cap.params, r#"[Int8(1), Float8(100.0), Text("%bot%")]"#);
}