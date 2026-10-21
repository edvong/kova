use crate::Result;
use crate::type_check::codec::AppCodec;
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Options;
use kova::dsl::LockMode;
use kova::dsl::func::{Count, Sum, Unnest, Wildcard};
use kova::query::Select;
use models::type_check::comment::Comment;
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::post::Post;
use models::type_check::user::User;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Stress Test 1: Multi-Step Financial & Content Pipeline inside an Async Transaction
/// Memo: Validates that `tx` (`Transaction`) seamlessly supports `insert`, `update`, `delete`,
/// `select` (with joins, locks, SRFs, and subqueries), and `statement`, piping returned IDs forward.
async fn compile_check_pg_transaction_pipeline() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let (final_user, post_id, remaining_balance, total_orders) = client
        .transaction(async |tx| {
            // Step 1: Upsert a new User and extract the generated primary key + initial balance
            let new_user = User {
                id: 0,
                name: "tx_pioneer".into(),
                bio: Some("initial".into()),
                balance: 1000.0,
                is_active: true,
            };

            let mut user_rows = tx
                .insert(&new_user)
                .on_conflict(User::id)
                .update((User::name.excluded(), User::balance.increment(500.0)))
                .returning((User::id, User::balance))
                .execute_in::<(i32, f64)>()
                .await?;
            let (user_id, current_balance) = user_rows.pop().unwrap();

            // Step 2: Pessimistically lock the User row (FOR UPDATE) inside the transaction
            let _locked = tx
                .select::<User>()
                .filter(User::id.eq(user_id))
                .lock::<User>(LockMode::Update, None)
                .load()
                .await?;

            // Step 3: Insert a Post authored by `user_id` via INSERT ... SELECT and grab the new post_id
            let mut post_ids = tx
                .insert::<Post>(
                    Select::from::<User>()
                        .columns((User::name, User::bio, User::id))
                        .filter(User::id.eq(user_id)),
                )
                .returning(Post::id)
                .execute_in::<i32>()
                .await?;
            let created_post_id = post_ids.pop().unwrap();

            // Step 4: Deduct order costs from User::balance and return the updated balance
            let order_cost = 250.50_f64;
            let mut updated_balances = tx
                .update::<User>()
                .set((
                    User::balance.decrement(order_cost),
                    User::bio.set("purchased_item"),
                ))
                .filter(User::id.eq(user_id) & User::balance.ge(order_cost))
                .returning(User::balance)
                .execute_in::<f64>()
                .await?;
            let new_balance = updated_balances.pop().unwrap_or(current_balance);

            // Step 5: Clean up stale comments on `created_post_id`
            let _deleted_count = tx
                .delete::<Comment>()
                .filter(Comment::post_id.eq(created_post_id) & Comment::user_id.ne(user_id))
                .execute()
                .await?;

            // Step 6: Aggregate order statistics using SRF + JOIN inside the transaction
            let badge_tiers = vec![1_i32, 2, 3];
            let mut stats = tx
                .select::<Order>()
                .srf(Unnest(&badge_tiers))
                .inner_join::<User>(Order::user_id.eq(User::id))
                .columns((Count(Wildcard), Sum(Order::amount)))
                .filter(Order::user_id.eq(user_id) & Order::id.ge(Unnest::unnest))
                .group_by(User::id)
                .load_in::<(i64, Option<f64>)>()
                .await?;
            let (order_cnt, _) = stats.pop().unwrap_or((0, None));

            // Step 7: Raw SQL audit log statement within the same transaction
            tx.statement("INSERT INTO audit_trail (user_id, post_id, balance) VALUES ($1, $2, $3)")
                .bind((user_id, created_post_id, new_balance))
                .execute()
                .await?;

            // Step 8: Fetch final User model to return out of the transaction
            let mut users = tx.select::<User>().filter(User::id.eq(user_id)).load().await?;
            Ok((users.pop().unwrap(), created_post_id, new_balance, order_cnt))
        })
        .await?;

    Ok(())
}

/// Stress Test 2: Custom Codec (`AppCodec`) Penetration inside Transaction
/// Memo: Proves that `Client<'_, Postgres, Async, AppCodec>::transaction` preserves the `AppCodec`
/// type parameter `X` across all inner operations without requiring manual type annotations.
async fn compile_check_pg_transaction_with_codec() -> Result<()> {
    let client = Options::from("").build()?.codec::<AppCodec>();

    let returned_url: Url = client
        .transaction(async |tx| {
            let log = EventLog {
                id: Uuid::new_v4(),
                description: "tx_init".into(),
                created_at: Utc::now(),
                updated_at: Timestamp::now(),
                website: Url::parse("https://rust-lang.org").unwrap(),
                metadata: HashMap::from([("step".into(), "1".into())]),
            };

            // 1. Insert EventLog and return custom `Uuid` + `DateTime<Utc>`
            let mut inserted = tx
                .insert(&log)
                .returning((EventLog::id, EventLog::created_at))
                .execute_in::<(Uuid, DateTime<Utc>)>()
                .await?;
            let (event_id, created_time) = inserted.pop().unwrap();

            // 2. Update using the returned `event_id` (by reference) and custom Compatible (&str -> Url)
            let mut urls = tx
                .update::<EventLog>()
                .set((
                    EventLog::website.set("https://docs.rs"),
                    EventLog::updated_at.set(Timestamp::now()),
                ))
                .filter(
                    EventLog::id.eq(&event_id)
                        & EventLog::created_at.ge(&created_time)
                        & EventLog::metadata.json_path_text("step").eq("1"),
                )
                .returning(EventLog::website)
                .execute_in::<Url>()
                .await?;

            Ok(urls.pop().unwrap())
        })
        .await?;

    Ok(())
}
