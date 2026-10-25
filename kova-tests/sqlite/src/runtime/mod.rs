pub mod client_patterns;
pub mod codec;
mod codec_and_types;
mod crud_and_batch;
mod queries;
mod tx_and_pool;

use kova::{Client, Options};
use crate::render::inspect_sql;

/// Creates an isolated single-connection in-memory SQLite `Client` with full runtime tables and indexes.
pub fn runtime_client() -> Client {
    Options::in_memory()
        .max_size(1)
        .init(|conn| {
            conn.regexp(|text, pat| {
                text.contains(pat.trim_start_matches('^').trim_end_matches('$'))
            })?;
            conn.execute_batch(
                r#"
                CREATE TABLE "user" (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "name" TEXT NOT NULL,
                    "bio" TEXT,
                    "balance" REAL NOT NULL,
                    "is_active" INTEGER NOT NULL
                );
                CREATE UNIQUE INDEX "idx_user_name_lower" ON "user" ("id", lower("name"));

                CREATE TABLE "order" (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
                    "user_id" INTEGER NOT NULL,
                    "amount" REAL NOT NULL,
                    "discount" REAL,
                    "is_active" INTEGER NOT NULL,
                    "raw_data" BLOB NOT NULL
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

                CREATE TABLE "event_log" (
                    "id" BLOB PRIMARY KEY,
                    "description" TEXT NOT NULL,
                    "created_at" INTEGER NOT NULL,
                    "updated_at" TEXT NOT NULL,
                    "website" TEXT NOT NULL,
                    "metadata" TEXT NOT NULL,
                    "cost" INTEGER NOT NULL DEFAULT 0,
                    "payload" TEXT NOT NULL DEFAULT '{"actor":"sys","level":1}',
                    "labels" TEXT NOT NULL DEFAULT '[]'
                );

                CREATE VIRTUAL TABLE "fts_doc" USING fts5("title", "body");
                "#,
            )
        })
        .inspect(inspect_sql)
        .build()
        .unwrap()
}