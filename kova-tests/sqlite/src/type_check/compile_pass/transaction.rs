use crate::Result;
use crate::type_check::codec::AppCodec;
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Options;
use kova::dsl::func::{Count, JsonEach, Sum, Wildcard};
use kova::query::Select;
use models::type_check::comment::Comment;
use models::type_check::event_log::EventLog;
use models::type_check::order::Order;
use models::type_check::post::Post;
use models::type_check::user::User;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Stress Test 1: Synchronous Multi-Step Transaction Pipeline on SQLite
/// Validates that synchronous `client.transaction(|tx| ...)` chains `insert -> returning`,
/// `update -> returning`, `delete`, `select` (with `.srf()`), and `statement` seamlessly.
fn compile_check_sqlite_transaction_pipeline() -> Result<()> {
    let client = Options::in_memory().build()?;

    let (final_user, post_id, remaining_balance, order_cnt) = client.transaction(|tx| {
        // Step 1: Insert User and grab generated ID + initial balance via RETURNING
        let new_user = User {
            id: 0,
            name: "sqlite_tx_user".into(),
            bio: Some("hello".into()),
            balance: 500.0,
            is_active: true,
        };

        let mut user_rows = tx
            .insert(&new_user)
            .on_conflict(User::id)
            .update((User::name.excluded(), User::balance.increment(100.0)))
            .returning((User::id, User::balance))
            .execute_in::<(i32, f64)>()?;
        let (user_id, initial_balance) = user_rows.pop().unwrap();

        // Step 2: Use `user_id` to insert a Post via INSERT ... SELECT and return the Post ID
        let mut post_ids = tx
            .insert::<Post>(
                Select::from::<User>()
                    .columns((User::name, User::bio, User::id))
                    .filter(User::id.eq(user_id)),
            )
            .returning(Post::id)
            .execute_in::<i32>()?;
        let created_post_id = post_ids.pop().unwrap();

        // Step 3: Deduct balance and return updated balance
        let cost = 125.0_f64;
        let mut balances = tx
            .update::<User>()
            .set((User::balance.decrement(cost), User::bio.set("updated_in_tx")))
            .filter(User::id.eq(user_id) & User::balance.ge(cost))
            .returning(User::balance)
            .execute_in::<f64>()?;
        let new_balance = balances.pop().unwrap_or(initial_balance);

        // Step 4: Delete old comments for `created_post_id`
        let _deleted = tx
            .delete::<Comment>()
            .filter(Comment::post_id.eq(created_post_id) & Comment::user_id.ne(user_id))
            .execute()?;

        // Step 5: Query with `.srf(JsonEach(...))` and aggregate inside the transaction
        let mut stats = tx
            .select::<Order>()
            .srf(JsonEach(User::name))
            .inner_join::<User>(Order::user_id.eq(User::id))
            .columns((Count(Wildcard), Sum(Order::amount)))
            .filter(Order::user_id.eq(user_id) & JsonEach::value.is_not_null())
            .group_by(User::id)
            .load_in::<(i64, Option<f64>)>()?;
        let (cnt, _) = stats.pop().unwrap_or((0, None));

        // Step 6: Raw SQL statement inside transaction
        tx.statement("INSERT INTO tx_audit (user_id, post_id, balance) VALUES (?, ?, ?)")
            .bind((user_id, created_post_id, new_balance))
            .execute()?;

        // Step 7: Load final User record
        let mut users = tx.select::<User>().filter(User::id.eq(user_id)).load()?;
        Ok((users.pop().unwrap(), created_post_id, new_balance, cnt))
    })?;

    Ok(())
}

/// Stress Test 2: Custom Codec (`AppCodec`) Penetration inside SQLite Transaction
fn compile_check_sqlite_transaction_with_codec() -> Result<()> {
    let client = Options::in_memory().build()?.codec::<AppCodec>();

    let final_url: Url = client.transaction(|tx| {
        let log = EventLog {
            id: Uuid::new_v4(),
            description: "sqlite_codec_tx".into(),
            created_at: Utc::now(),
            updated_at: Timestamp::now(),
            website: Url::parse("https://sqlite.org").unwrap(),
            metadata: HashMap::from([("env".into(), "test".into())]),
        };

        // 1. Insert and return custom `Uuid` and `DateTime<Utc>`
        let mut inserted = tx
            .insert(&log)
            .returning((EventLog::id, EventLog::created_at))
            .execute_in::<(Uuid, DateTime<Utc>)>()?;
        let (event_id, created_at) = inserted.pop().unwrap();

        // 2. Update using returned `event_id` and custom Compatible (&str -> Url)
        let mut urls = tx
            .update::<EventLog>()
            .set((
                EventLog::website.set("https://rust-lang.org"),
                EventLog::updated_at.set(Timestamp::now()),
            ))
            .filter(
                EventLog::id.eq(&event_id)
                    & EventLog::created_at.ge(&created_at)
                    & EventLog::metadata.json_extract("$.env").eq("test"),
            )
            .returning(EventLog::website)
            .execute_in::<Url>()?;

        Ok(urls.pop().unwrap())
    })?;

    Ok(())
}
