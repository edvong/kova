use crate::render::capture_all;
use crate::runtime::runtime_client;
use kova::{Error, Result};
use kova::dsl::Constraint;
use kova::dsl::func::{Count, Lower, Wildcard};
use kova::query::Select;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};
use models::runtime::order::Order;
use models::runtime::post::Post;
use models::runtime::user::User;

// Dummy model with zero insertable columns to verify the empty-column guard in `InsertStatement::render_batches`.
pub struct EmptyPgModel;

impl Relation for EmptyPgModel {
    const NAME: &'static str = "user";
}

impl<X> FromRow<Engine, X> for EmptyPgModel {
    const COLUMN_COUNT: usize = 0;
    
    fn from_row_at(_: &<Engine as Database>::Row<'_>, _idx: &mut usize) -> Result<Self> {
        Ok(Self)
    }
}

impl<X> Model<Engine, X> for EmptyPgModel {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn write_values<'a>(&'a self, _out: &mut Vec<<Engine as Database>::Value<'a>>) -> Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn pg_full_crud_raw_load_as_and_upsert_lifecycle() {
    let client = runtime_client().await;

    let u1 = User {
        id: 0,
        name: "Alice".into(),
        bio: Some("Engineer".into()),
        balance: 500.0,
        is_active: true,
    };
    let u2 = User {
        id: 0,
        name: "Bob".into(),
        bio: None,
        balance: 100.0,
        is_active: false,
    };

    // 1. Batch insert + Returning Model
    let inserted = client
        .insert(&[u1, u2][..])
        .returning((User::id, User::name, User::bio, User::balance, User::is_active))
        .execute_as::<User>()
        .await
        .unwrap();
    assert_eq!(inserted.len(), 2);
    assert_eq!(inserted[0].id, 1);
    assert_eq!(inserted[1].id, 2);

    // 2. RawStatement `.load_as::<User>()`
    let raw_users = client
        .statement(r#"SELECT "id", "name", "bio", "balance", "is_active" FROM "user" WHERE "id" = $1"#)
        .bind(1_i32)
        .load_as::<User>()
        .await
        .unwrap();
    assert_eq!(raw_users, vec![inserted[0].clone()]);

    // 3. ON CONFLICT ON CONSTRAINT "uk_user_email_idx" DO NOTHING
    let dup_alice = User {
        id: 0,
        name: "Alice".into(),
        bio: Some("Ignored".into()),
        balance: 9999.0,
        is_active: true,
    };
    let affected = client
        .insert(&dup_alice)
        .on_conflict(Constraint("uk_user_email_idx"))
        .do_nothing()
        .execute()
        .await
        .unwrap();
    assert_eq!(affected, 0);

    // 4. ON CONFLICT (name) DO UPDATE SET excluded + increment + WHERE
    let updated_row = client
        .insert(&dup_alice)
        .on_conflict(User::name)
        .update((User::bio.excluded(), User::balance.increment(250.0)))
        .filter(User::is_active.eq(true))
        .returning((User::id, Lower(User::name), User::bio, User::balance))
        .execute_in::<(i32, String, Option<String>, f64)>()
        .await
        .unwrap();
    assert_eq!(
        updated_row,
        vec![(1, "alice".to_string(), Some("Ignored".to_string()), 750.0)]
    );

    // 5. UPDATE ... FROM ... RETURNING
    client
        .insert(Order {
            id: 0,
            user_id: 1,
            amount: 50.0,
            discount: Some(5.0),
            is_active: true,
            raw_data: vec![1, 2],
        })
        .execute()
        .await
        .unwrap();

    let new_balances = client
        .update::<User>()
        .set(User::balance.decrement(Order::amount))
        .from::<Order>()
        .filter(User::id.eq(Order::user_id) & Order::is_active.eq(true))
        .returning(User::balance)
        .execute_in::<f64>()
        .await
        .unwrap();
    assert_eq!(new_balances, vec![700.0]);

    // 6. INSERT ... SELECT
    let post_ids = client
        .insert::<Post>(
            Select::from::<User>()
                .columns((User::name, User::name, User::id))
                .filter(User::id.eq(1)),
        )
        .returning(Post::id)
        .execute_in::<i32>()
        .await
        .unwrap();
    assert_eq!(post_ids, vec![1]);

    // 7. DELETE ... RETURNING
    let deleted = client
        .delete::<User>()
        .filter(User::is_active.eq(false))
        .returning(User::name)
        .execute_in::<String>()
        .await
        .unwrap();
    assert_eq!(deleted, vec!["Bob".to_string()]);
}

#[tokio::test]
async fn pg_insert_model_with_zero_columns_returns_query_error() {
    let client = runtime_client().await;

    let err = client.insert(EmptyPgModel).execute().await.unwrap_err();
    assert!(
        matches!(err, Error::QueryError(ref msg) if msg.contains("INSERT requires at least one column value")),
        "unexpected error: {err:?}"
    );
}

#[tokio::test]
async fn pg_insert_exceeding_65535_max_params_splits_batches() {
    let client = runtime_client().await;

    // Postgres::MAX_PARAMS = 65535. User has 4 non-auto columns ("name", "bio", "balance", "is_active").
    // Max rows per batch = 65535 / 4 = 16383.
    // Inserting 16390 rows splits into 2 batches (16383 rows + 7 rows) and resets placeholders to $1 in batch 2.
    let rows: Vec<User> = (0..16390)
        .map(|i| User {
            id: 0,
            name: format!("pg_user_{i}"),
            bio: None,
            balance: i as f64,
            is_active: true,
        })
        .collect();

    let (res, logs) = capture_all(client.insert(&rows).execute()).await;
    assert_eq!(res.unwrap(), 16390);
    assert_eq!(logs.len(), 2, "16390 rows * 4 cols must split into 2 batches");
    assert!(logs[1].sql.contains("values ($1, $2, $3, $4)"));

    let total = client
        .select::<User>()
        .columns(Count(Wildcard))
        .load_in::<i64>()
        .await
        .unwrap();
    assert_eq!(total, vec![16390]);
}