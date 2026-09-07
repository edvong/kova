use crate::types::{Sqlite, SqlitePool};
use kova_core::database::Client;
use kova_core::{Error, Result};
use r2d2::Builder;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::path::Path;
use std::time::Duration;

pub struct Options {
    manager: SqliteConnectionManager,
    pool_builder: Builder<SqliteConnectionManager>,
}

impl Options {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        Self {
            manager: SqliteConnectionManager::file(path),
            pool_builder: r2d2::Pool::builder(),
        }
    }

    pub fn in_memory() -> Self {
        Self {
            manager: SqliteConnectionManager::memory(),
            pool_builder: r2d2::Pool::builder(),
        }
    }

    pub fn max_size(mut self, v: u32) -> Self {
        self.pool_builder = self.pool_builder.max_size(v);
        self
    }

    pub fn min_idle(mut self, v: u32) -> Self {
        self.pool_builder = self.pool_builder.min_idle(Some(v));
        self
    }

    pub fn max_lifetime(mut self, v: Duration) -> Self {
        self.pool_builder = self.pool_builder.max_lifetime(Some(v));
        self
    }

    pub fn idle_timeout(mut self, v: Duration) -> Self {
        self.pool_builder = self.pool_builder.idle_timeout(Some(v));
        self
    }

    pub fn connection_timeout(mut self, v: Duration) -> Self {
        self.pool_builder = self.pool_builder.connection_timeout(v);
        self
    }

    pub fn test_on_check_out(mut self, v: bool) -> Self {
        self.pool_builder = self.pool_builder.test_on_check_out(v);
        self
    }

    pub fn init<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Connection) -> Result<(), rusqlite::Error> + Send + Sync + 'static,
    {
        self.manager = self.manager.with_init(f);
        self
    }

    pub fn build(self) -> Result<Client<'static, Sqlite, kova_core::types::Sync, ()>> {
        let pool = self
            .pool_builder
            .build(self.manager)
            .map_err(|e| Error::Database(Box::new(e)))?;

        Ok(Client::new(SqlitePool(pool)))
    }
}