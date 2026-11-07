use crate::types::conn::{InitFn, Target};
use crate::types::{Inspector, Sqlite, SqlitePool};
use kova_kernel::database::Client;
use kova_kernel::{Error, Result};
use rusqlite::Connection;
use rusqlite::functions::FunctionFlags;
use std::path::Path;
use std::time::Duration;

pub struct InitConnection<'a>(pub(crate) &'a mut Connection);

impl<'a> InitConnection<'a> {
    pub fn execute_batch(&self, sql: &str) -> Result<()> {
        self.0.execute_batch(sql).map_err(Error::database)
    }

    /// Registers SQLite's `X REGEXP Y` operator callback `f(text, pattern)`.
    pub fn regexp<F>(&self, f: F) -> Result<()>
        where
            F: Fn(&str, &str) -> bool + Send + Sync + 'static {
        self.0
            .create_scalar_function(
                "regexp",
                2,
                FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
                move |ctx| {
                    let pattern = ctx.get_raw(0).as_str().map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;
                    let text = ctx.get_raw(1).as_str().map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            1,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;
                    Ok(f(text, pattern))
                },
            )
            .map_err(Error::database)
    }
}

pub struct Options {
    target: Target,
    init: Option<InitFn>,
    max_size: u32,
    timeout: Option<Duration>,
    inspector: Option<Inspector>,
}

impl Options {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        Self {
            target: Target::File(path.as_ref().to_path_buf()),
            init: None,
            max_size: 8,
            timeout: Some(Duration::from_secs(30)),
            inspector: None,
        }
    }

    pub fn in_memory() -> Self {
        Self {
            target: Target::Memory,
            init: None,
            max_size: 1,
            timeout: Some(Duration::from_secs(30)),
            inspector: None,
        }
    }

    pub fn inspect(mut self, f: Inspector) -> Self {
        self.inspector = Some(f);
        self
    }

    pub fn max_size(mut self, v: u32) -> Self {
        self.max_size = v;
        self
    }

    pub fn connection_timeout(mut self, v: Duration) -> Self {
        self.timeout = Some(v);
        self
    }

    pub fn init<F>(mut self, f: F) -> Self
        where
            F: Fn(&mut InitConnection<'_>) -> Result<()> + Send + Sync + 'static {
        self.init = Some(Box::new(f));
        self
    }

    pub fn build(self) -> Result<Client<Sqlite, kova_kernel::types::Sync, ()>> {
        let pool =
            SqlitePool::new(self.target, self.init, self.max_size, self.timeout, self.inspector)?;
        Ok(Client::new(pool))
    }
}
