mod clauses_and_locks;
mod conflict_and_set;
mod json_array_srf;
mod proptest_ast;
mod window_and_tags;

use kova::codec::Value;
use kova::{Client, Options};
use std::cell::RefCell;
use std::future::Future;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedQuery {
    pub sql: String,
    pub params: String,
}

tokio::task_local! {
    static RECORDS: RefCell<Vec<CapturedQuery>>;
}

fn inspect_sql(sql: &str, params: &[Value<'_>]) {
    let _ = RECORDS.try_with(|r| {
        r.borrow_mut().push(CapturedQuery {
            sql: sql.to_string(),
            params: format!("{params:?}"),
        });
    });
}

pub async fn capture_all<F, R>(fut: F) -> (R, Vec<CapturedQuery>)
where
    F: Future<Output = R>,
{
    RECORDS
        .scope(RefCell::new(Vec::new()), async move {
            let result = fut.await;
            let captured = RECORDS.with(|r| std::mem::take(&mut *r.borrow_mut()));
            (result, captured)
        })
        .await
}

/// Captures a single SQL statement and asserts that the PostgreSQL server parsed and executed it without error.
pub async fn capture_one<F, T>(fut: F) -> CapturedQuery
where
    F: Future<Output = kova::Result<T>>,
{
    let (res, mut logs) = capture_all(fut).await;
    assert!(
        res.is_ok(),
        "PostgreSQL engine rejected the rendered SQL query: {:?}\nCaptured logs: {:?}",
        res.err(),
        logs
    );
    assert_eq!(
        logs.len(),
        1,
        "expected exactly 1 SQL event, captured {}: {:?}",
        logs.len(),
        logs
    );
    logs.remove(0)
}

pub fn pg_conn_str() -> String {
    std::env::var("KOVA_PG_URL").unwrap_or_else(|_| {
        "host=127.0.0.1 port=54329 user=postgres password=password dbname=kova_test".to_string()
    })
}

/// Creates a `max_size(1)` PostgreSQL `Client` with session-private `TEMP TABLE`s (`pg_temp` schema).
/// Because `pg_temp` takes precedence over `public` in PostgreSQL's `search_path`, every test gets
/// an isolated table namespace and can run in parallel without table or row collisions.
pub async fn render_client() -> Client {
    let client = Options::from(pg_conn_str())
        .max_size(1)
        .inspect(inspect_sql)
        .build()
        .expect("failed to build Postgres client");

    let ddl = [
        r#"CREATE TEMP TABLE "user" (
            "id" SERIAL PRIMARY KEY,
            "name" TEXT NOT NULL,
            "bio" TEXT,
            "balance" DOUBLE PRECISION NOT NULL,
            "is_active" BOOLEAN NOT NULL,
            CONSTRAINT "uk_user_email_idx" UNIQUE ("name")
        )"#,
        r#"CREATE UNIQUE INDEX ON "user" ("id", lower("name"))"#,
        r#"CREATE TEMP TABLE "order" (
            "id" SERIAL PRIMARY KEY,
            "user_id" INTEGER NOT NULL,
            "amount" DOUBLE PRECISION NOT NULL,
            "discount" DOUBLE PRECISION,
            "is_active" BOOLEAN NOT NULL,
            "raw_data" BYTEA NOT NULL,
            "created_at" INTEGER DEFAULT 0
        )"#,
        r#"CREATE TEMP TABLE "post" (
            "id" SERIAL PRIMARY KEY,
            "title" TEXT NOT NULL,
            "content" TEXT NOT NULL,
            "author_id" INTEGER NOT NULL,
            "tags" TEXT[] DEFAULT '{}'
        )"#,
        r#"CREATE TEMP TABLE "comment" (
            "id" SERIAL PRIMARY KEY,
            "post_id" INTEGER NOT NULL,
            "user_id" INTEGER NOT NULL,
            "content" TEXT NOT NULL
        )"#,
        r#"CREATE TEMP TABLE "event_log" (
            "id" BYTEA PRIMARY KEY,
            "description" TEXT NOT NULL,
            "created_at" BIGINT NOT NULL,
            "updated_at" TEXT NOT NULL,
            "website" TEXT NOT NULL,
            "metadata" JSONB NOT NULL,
            "opt_website" TEXT,
            "cost" BIGINT NOT NULL DEFAULT 0,
            "opt_cost" BIGINT,
            "payload" JSONB NOT NULL DEFAULT '{}'::jsonb,
            "opt_payload" JSONB,
            "labels" TEXT[] NOT NULL DEFAULT '{}',
            "opt_labels" TEXT[]
        )"#,
    ];

    for sql in ddl {
        client.statement(sql).execute().await.unwrap();
    }

    client
}