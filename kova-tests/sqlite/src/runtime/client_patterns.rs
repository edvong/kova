use crate::runtime::codec::AppCodec;
use crate::runtime::runtime_client;
use kova::{Client, Options};
use models::runtime::order::Order;
use models::runtime::user::User;
use std::sync::LazyLock;
use std::thread;

// ==========================================
// Pattern 1: Global `LazyLock<Client<AppCodec>>` (`DB` & lowercase `client`)
// ==========================================
pub static DB: LazyLock<Client<AppCodec>> = LazyLock::new(|| {
    Options::in_memory()
        .max_size(1)
        .init(|conn| {
            conn.execute_batch(
                r#"
                CREATE TABLE "order" (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "user_id" INTEGER NOT NULL,
                    "amount" REAL NOT NULL,
                    "discount" REAL,
                    "is_active" INTEGER NOT NULL,
                    "raw_data" BLOB NOT NULL
                );
                "#,
            )
        })
        .build()
        .unwrap()
        .codec::<AppCodec>()
});

#[allow(non_upper_case_globals)]
pub static client: LazyLock<Client<AppCodec>> = LazyLock::new(|| {
    Options::in_memory()
        .max_size(1)
        .init(|conn| {
            conn.execute_batch(
                r#"
                CREATE TABLE "user" (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "name" TEXT NOT NULL,
                    "bio" TEXT,
                    "balance" REAL NOT NULL,
                    "is_active" INTEGER NOT NULL
                );
                "#,
            )
        })
        .build()
        .unwrap()
        .codec::<AppCodec>()
});

fn create_order(order: &Order) -> kova::Result<i32> {
    let mut ids = DB.insert(order).returning(Order::id).execute_in::<i32>()?;
    Ok(ids.pop().unwrap())
}

// ==========================================
// Pattern 2: Struct field holding `Client<AppCodec>` with `#[derive(Clone)]`
// Note: `AppCodec` does NOT implement `Clone`, proving `Client`'s manual `Clone` impl works.
// ==========================================
#[derive(Clone)]
pub struct OrderRepo {
    client: Client<AppCodec>,
}

impl OrderRepo {
    pub fn new(c: Client<AppCodec>) -> Self {
        Self { client: c }
    }

    pub fn create(&self, order: &Order) -> kova::Result<i32> {
        let mut ids = self
            .client
            .insert(order)
            .returning(Order::id)
            .execute_in::<i32>()?;
        Ok(ids.pop().unwrap())
    }

    pub fn find_by_id(&self, id: i32) -> kova::Result<Vec<Order>> {
        self.client
            .select::<Order>()
            .filter(Order::id.eq(id))
            .load()
    }
}

#[test]
fn static_lazylock_db_and_lowercase_client_pattern() {
    let order = Order {
        id: 0,
        user_id: 7,
        amount: 256.0,
        discount: Some(12.5),
        is_active: true,
        raw_data: vec![1, 2, 3],
    };

    let inserted_id = create_order(&order).unwrap();
    let loaded = DB
        .select::<Order>()
        .filter(Order::id.eq(inserted_id))
        .load()
        .unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].amount, 256.0);

    // Lowercase `client` static LazyLock accessed directly via Deref
    client
        .insert(User {
            id: 0,
            name: "lazy_static_user".into(),
            bio: None,
            balance: 42.0,
            is_active: true,
        })
        .execute()
        .unwrap();

    let names = client
        .select::<User>()
        .columns(User::name)
        .filter(User::name.eq("lazy_static_user"))
        .load_in::<String>()
        .unwrap();
    assert_eq!(names, vec!["lazy_static_user".to_string()]);
}

#[test]
fn repository_struct_derive_clone_and_multithreaded_pool_sharing() {
    let base_client = runtime_client().codec::<AppCodec>();
    let repo = OrderRepo::new(base_client);

    // Clone the repository across multiple worker threads and verify they share the same pool.
    let mut handles = Vec::new();
    for i in 1..=4 {
        let repo_clone = repo.clone();
        handles.push(thread::spawn(move || {
            let order = Order {
                id: 0,
                user_id: i,
                amount: (i * 100) as f64,
                discount: None,
                is_active: true,
                raw_data: vec![i as u8],
            };
            let id = repo_clone.create(&order).unwrap();
            let found = repo_clone.find_by_id(id).unwrap();
            assert_eq!(found.len(), 1);
            assert_eq!(found[0].user_id, i);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Original repo sees all 4 orders inserted by the cloned instances.
    let all_orders = repo.client.select::<Order>().load().unwrap();
    assert_eq!(all_orders.len(), 4);
}