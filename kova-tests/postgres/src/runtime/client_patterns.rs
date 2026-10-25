use crate::render::pg_conn_str;
use crate::runtime::codec::AppCodec;
use crate::runtime::runtime_client;
use kova::{Client, Options};
use models::runtime::order::Order;
use models::runtime::user::User;
use std::sync::LazyLock;

// ==========================================
// Pattern 1: Global `LazyLock<Client<AppCodec>>` (`DB` & lowercase `client`)
// ==========================================
pub static DB: LazyLock<Client<AppCodec>> = LazyLock::new(|| {
    Options::from(pg_conn_str())
        .max_size(4)
        .build()
        .unwrap()
        .codec::<AppCodec>()
});

#[allow(non_upper_case_globals)]
pub static client: LazyLock<Client<AppCodec>> = LazyLock::new(|| {
    Options::from(pg_conn_str())
        .max_size(4)
        .build()
        .unwrap()
        .codec::<AppCodec>()
});

async fn create_order(order: &Order) -> kova::Result<i32> {
    let mut ids = DB
        .insert(order)
        .returning(Order::id)
        .execute_in::<i32>()
        .await?;
    Ok(ids.pop().unwrap())
}

// ==========================================
// Pattern 2: Repository / Service holding `Client<AppCodec>` with `#[derive(Clone)]`
// ==========================================
#[derive(Clone)]
pub struct OrderRepo {
    client: Client<AppCodec>,
}

impl OrderRepo {
    pub fn new(c: Client<AppCodec>) -> Self {
        Self { client: c }
    }

    pub async fn create(&self, order: &Order) -> kova::Result<i32> {
        let mut ids = self
            .client
            .insert(order)
            .returning(Order::id)
            .execute_in::<i32>()
            .await?;
        Ok(ids.pop().unwrap())
    }

    pub async fn find_by_id(&self, id: i32) -> kova::Result<Vec<Order>> {
        self.client
            .select::<Order>()
            .filter(Order::id.eq(id))
            .load()
            .await
    }
}

#[tokio::test]
async fn pg_static_lazylock_db_and_lowercase_client_pattern() {
    // Initialize public tables for the multi-connection static LazyLock pools.
    // Note: Other tests use session-local `TEMP TABLE`s (`pg_temp`), which shadow `public`
    // in PostgreSQL's `search_path`, so `public."order"` and `public."user"` never collide with them.
    DB.statement(
        r#"CREATE TABLE IF NOT EXISTS public."order" (
            "id" SERIAL PRIMARY KEY,
            "user_id" INTEGER NOT NULL,
            "amount" DOUBLE PRECISION NOT NULL,
            "discount" DOUBLE PRECISION,
            "is_active" BOOLEAN NOT NULL,
            "raw_data" BYTEA NOT NULL,
            "created_at" INTEGER DEFAULT 0
        )"#,
    )
        .execute()
        .await
        .unwrap();

    client
        .statement(
            r#"CREATE TABLE IF NOT EXISTS public."user" (
                "id" SERIAL PRIMARY KEY,
                "name" TEXT NOT NULL,
                "bio" TEXT,
                "balance" DOUBLE PRECISION NOT NULL,
                "is_active" BOOLEAN NOT NULL
            )"#,
        )
        .execute()
        .await
        .unwrap();

    let order = Order {
        id: 0,
        user_id: 9001,
        amount: 512.75,
        discount: Some(10.0),
        is_active: true,
        raw_data: vec![0xAA, 0xBB],
    };

    let order_id = create_order(&order).await.unwrap();
    let repo = OrderRepo::new(DB.clone());
    let loaded = repo.find_by_id(order_id).await.unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].user_id, 9001);
    assert_eq!(loaded[0].amount, 512.75);

    // Lowercase static `client` used directly across async tasks
    let mut user_ids = client
        .insert(User {
            id: 0,
            name: "pg_lazy_user".into(),
            bio: None,
            balance: 88.0,
            is_active: true,
        })
        .returning(User::id)
        .execute_in::<i32>()
        .await
        .unwrap();
    let uid = user_ids.pop().unwrap();

    let names = client
        .select::<User>()
        .columns(User::name)
        .filter(User::id.eq(uid))
        .load_in::<String>()
        .await
        .unwrap();
    assert_eq!(names, vec!["pg_lazy_user".to_string()]);

    // Clean up inserted rows
    DB.delete::<Order>().filter(Order::id.eq(order_id)).execute().await.unwrap();
    client.delete::<User>().filter(User::id.eq(uid)).execute().await.unwrap();
}

#[tokio::test]
async fn pg_order_repo_derive_clone_across_concurrent_tokio_tasks() {
    let base_client = runtime_client().await.codec::<AppCodec>();
    let repo = OrderRepo::new(base_client);

    let mut join_handles = Vec::new();
    for i in 1..=6 {
        let repo_clone = repo.clone();
        join_handles.push(tokio::spawn(async move {
            let order = Order {
                id: 0,
                user_id: i,
                amount: (i * 50) as f64,
                discount: None,
                is_active: true,
                raw_data: vec![i as u8],
            };
            let id = repo_clone.create(&order).await.unwrap();
            let fetched = repo_clone.find_by_id(id).await.unwrap();
            assert_eq!(fetched.len(), 1);
            assert_eq!(fetched[0].user_id, i);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }

    let all = repo.client.select::<Order>().load().await.unwrap();
    assert_eq!(all.len(), 6);
}