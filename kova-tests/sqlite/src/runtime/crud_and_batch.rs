use crate::render::capture_all;
use crate::runtime::runtime_client;
use kova::{Error, Result};
use kova::dsl::func::{Count, Lower, Wildcard};
use kova::query::Select;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};
use models::runtime::order::Order;
use models::runtime::post::Post;
use models::runtime::user::User;

// Dummy model with zero insertable columns to test the empty-column guard in `InsertStatement::render_batches`.
pub struct EmptyColumnsModel;

impl Relation for EmptyColumnsModel {
    const NAME: &'static str = "user";
}

impl<X> FromRow<Engine, X> for EmptyColumnsModel {
    const COLUMN_COUNT: usize = 0;

    fn from_row_at(_: &<Engine as Database>::Row<'_>, _idx: &mut usize) -> Result<Self> {
        Ok(Self)
    }
}

impl<X> Model<Engine, X> for EmptyColumnsModel {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn write_values<'a>(&'a self, _out: &mut Vec<<Engine as Database>::Value<'a>>) -> Result<()> {
        Ok(())
    }
}

#[test]
fn full_crud_and_returning_lifecycle() {
    let client = runtime_client();

    let alice = User {
        id: 0,
        name: "Alice".into(),
        bio: Some("Rustacean".into()),
        balance: 1000.0,
        is_active: true,
    };
    let bob = User {
        id: 0,
        name: "Bob".into(),
        bio: None,
        balance: 250.5,
        is_active: false,
    };

    // 1. Insert + Returning Model
    let inserted = client
        // .insert(vec![&alice, &bob]) // The &User does not implement Model

        // Solution A: Direct Move Ownership Vec<User>
        .insert(vec![alice, bob])
        // Solution B: Pass in consecutive slice references &[User]
        // .insert(&[alice, bob][..])
        .returning((User::id, User::name, User::bio, User::balance, User::is_active))
        .execute_as::<User>()
        .unwrap();
    assert_eq!(inserted.len(), 2);
    assert_eq!(inserted[0].id, 1);
    assert_eq!(inserted[0].name, "Alice");
    assert_eq!(inserted[1].id, 2);
    assert_eq!(inserted[1].bio, None);

    // 2. RawStatement `.load_as::<User>()` and primary key conflict check
    let raw_loaded = client
        .statement(r#"SELECT "id", "name", "bio", "balance", "is_active" FROM "user" WHERE "id" = ?"#)
        .bind(1_i32)
        .load_as::<User>()
        .unwrap();
    assert_eq!(raw_loaded, vec![inserted[0].clone()]);

    // 3. Upsert with ON CONFLICT + DO UPDATE + WHERE
    client
        .statement(r#"CREATE UNIQUE INDEX "uk_user_name" ON "user" ("name")"#)
        .execute()
        .unwrap();

    let conflict_alice = User {
        id: 0,
        name: "ALICE".into(),
        bio: Some("Updated Bio".into()),
        balance: 9999.0,
        is_active: true,
    };

    let first_upsert = client
        .insert(&conflict_alice)
        .on_conflict(User::name)
        .update((User::bio.excluded(), User::balance.increment(50.0)))
        .returning(User::balance)
        .execute_in::<f64>()
        .unwrap();
    assert_eq!(first_upsert, vec![9999.0]);

    let second_upsert = client
        .insert(&conflict_alice)
        .on_conflict(User::name)
        .update((User::bio.excluded(), User::balance.increment(50.0)))
        .filter(User::is_active.eq(true))
        .returning((User::id, Lower(User::name), User::balance))
        .execute_in::<(i32, String, f64)>()
        .unwrap();
    assert_eq!(second_upsert, vec![(3, "alice".to_string(), 10049.0)]);

    // 4. Update + Increment/Decrement + Returning
    let updated = client
        .update::<User>()
        .set((
            User::balance.decrement(49.0),
            User::bio.set(None::<String>),
        ))
        .filter(User::id.eq(3))
        .returning((User::balance, User::bio))
        .execute_in::<(f64, Option<String>)>()
        .unwrap();
    assert_eq!(updated, vec![(10000.0, None)]);

    // 5. INSERT ... SELECT
    let copied = client
        .insert::<Post>(
            Select::from::<User>()
                .columns((User::name, User::name, User::id))
                .filter(User::id.le(2)),
        )
        .returning((Post::id, Post::title, Post::author_id))
        .execute_in::<(i32, String, i32)>()
        .unwrap();
    assert_eq!(
        copied,
        vec![(1, "Alice".to_string(), 1), (2, "Bob".to_string(), 2)]
    );

    // 6. Delete + Returning
    let deleted_ids = client
        .delete::<User>()
        .filter(User::is_active.eq(false))
        .returning(User::id)
        .execute_in::<i32>()
        .unwrap();
    assert_eq!(deleted_ids, vec![2]);
}

#[test]
fn insert_model_with_zero_columns_returns_query_error() {
    let client = runtime_client();

    let err = client.insert(EmptyColumnsModel).execute().unwrap_err();
    assert!(
        matches!(err, Error::QueryError(ref msg) if msg.contains("INSERT requires at least one column value")),
        "unexpected error: {err:?}"
    );
}

#[test]
fn insert_exceeding_max_params_splits_into_multiple_batches() {
    let client = runtime_client();

    // SQLite::MAX_PARAMS = 32766. User has 4 non-auto columns ("name", "bio", "balance", "is_active").
    // Max rows per batch = 32766 / 4 = 8191.
    // 8200 rows must split into 2 batches (8191 rows + 9 rows).
    let rows: Vec<User> = (0..8200)
        .map(|i| User {
            id: 0,
            name: format!("u_{i}"),
            bio: None,
            balance: i as f64,
            is_active: i % 2 == 0,
        })
        .collect();

    let (res, logs) = capture_all(|| client.insert(&rows).execute());
    assert_eq!(res.unwrap(), 8200);
    assert_eq!(logs.len(), 2, "8200 rows * 4 cols must split into 2 batches");

    let count = client
        .select::<User>()
        .columns(Count(Wildcard))
        .load_in::<i64>()
        .unwrap();
    assert_eq!(count, vec![8200]);
}

#[test]
fn update_from_multi_table_execution() {
    let client = runtime_client();

    let u_id = client
        .insert(User {
            id: 0,
            name: "vip".into(),
            bio: None,
            balance: 500.0,
            is_active: true,
        })
        .returning(User::id)
        .execute_in::<i32>()
        .unwrap()[0];

    client
        .insert(Order {
            id: 0,
            user_id: u_id,
            amount: 100.0,
            discount: None,
            is_active: true,
            raw_data: vec![1, 2, 3],
        })
        .execute()
        .unwrap();

    let affected = client
        .update::<Order>()
        .set(Order::amount.set(88.8))
        .from::<User>()
        .filter(Order::user_id.eq(User::id) & User::name.eq("vip"))
        .execute()
        .unwrap();
    assert_eq!(affected, 1);

    let amt = client
        .select::<Order>()
        .columns(Order::amount)
        .load_in::<f64>()
        .unwrap();
    assert_eq!(amt, vec![88.8]);
}