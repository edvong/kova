mod clauses;
mod conflict_and_set;
mod expressions;
mod json_fts_srf;
mod proptest_ast;
mod tags;

use kova::codec::Value;
use kova::{Client, Options};
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedQuery {
    pub sql: String,
    pub params: String,
}

thread_local! {
    static RECORDS: RefCell<Vec<CapturedQuery>> = const { RefCell::new(Vec::new()) };
}

pub fn inspect_sql(sql: &str, params: &[Value<'_>]) {
    RECORDS.with_borrow_mut(|r| {
        r.push(CapturedQuery {
            sql: sql.to_string(),
            params: format!("{params:?}"),
        });
    });
}

pub fn capture_all<F, R>(f: F) -> (R, Vec<CapturedQuery>)
    where
        F: FnOnce() -> R {
    RECORDS.with_borrow_mut(|r| r.clear());
    let result = f();
    let captured = RECORDS.with_borrow_mut(std::mem::take);
    (result, captured)
}

/// Captures a single SQL statement and asserts that the underlying SQLite engine accepted and executed it.
pub fn capture_one<F, T>(f: F) -> CapturedQuery
    where
        F: FnOnce() -> kova::Result<T> {
    let (res, mut logs) = capture_all(f);
    assert!(
        res.is_ok(),
        "SQLite engine rejected the rendered SQL query: {:?}\nCaptured logs: {:?}",
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

/// Builds an isolated single-connection in-memory SQLite `Client` pre-initialized with test tables.
pub fn render_client() -> Client {
    Options::in_memory()
        .max_size(1)
        .init(|conn| {
            conn.regexp(|_pat, _text| true)?;
            conn.execute_batch(
                r#"
                CREATE TABLE "user" (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "name" TEXT NOT NULL,
                    "bio" TEXT,
                    "balance" REAL NOT NULL,
                    "is_active" INTEGER NOT NULL
                );
                CREATE UNIQUE INDEX "idx_user_active_lower_name" ON "user" ("is_active", lower("name"));

                CREATE TABLE "order" (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "user_id" INTEGER NOT NULL,
                    "amount" REAL NOT NULL,
                    "discount" REAL,
                    "is_active" INTEGER NOT NULL,
                    "raw_data" BLOB NOT NULL,
                    "created_at" INTEGER DEFAULT 0
                );
                CREATE TABLE "post" (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "title" TEXT NOT NULL,
                    "content" TEXT NOT NULL,
                    "author_id" INTEGER NOT NULL
                );
                CREATE TABLE "comment" (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "post_id" INTEGER NOT NULL,
                    "user_id" INTEGER NOT NULL,
                    "content" TEXT NOT NULL
                );
                CREATE VIRTUAL TABLE "event_log" USING fts5(
                    "id",
                    "description",
                    "created_at",
                    "updated_at",
                    "website",
                    "metadata",
                    "cost",
                    "payload",
                    "labels"
                );
                "#,
            )
        })
        .inspect(inspect_sql)
        .build()
        .unwrap()
}