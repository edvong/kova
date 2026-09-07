use kova_core::types::{Connection, ConnectionSync, Executable, ExecutableSync, Sync};
use kova_core::{Error, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub struct SqlitePool(pub Pool<SqliteConnectionManager>);

impl Executable for SqlitePool {
    type Mode = Sync;
    type Native = rusqlite::Connection;
}

impl ExecutableSync for SqlitePool {
    fn with_native_sync<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Self::Native) -> Result<R>
    {
        let guard = self.0.get().map_err(|e| Error::Database(e.into()))?;
        f(&*guard)
    }
}


pub struct SqliteTx<'t>(rusqlite::Transaction<'t>);

impl<'a> Executable for SqliteTx<'a> {
    type Mode = Sync;
    type Native = rusqlite::Connection;
}

impl<'a> ExecutableSync for SqliteTx<'a> {
    fn with_native_sync<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Self::Native) -> Result<R>
    {
        // rusqlite::Transaction implemented Deref<Target = Connection>
        f(&self.0)
    }
}

impl Connection for SqlitePool {
    type Transaction<'t> = SqliteTx<'t>;
}

impl<'a> ConnectionSync for SqlitePool {

    fn transaction_sync<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self::Transaction<'_>) -> Result<R>
    {
        let mut guard = self.0.get().map_err(|e| Error::Database(e.into()))?;
        let native_tx = guard.transaction().map_err(|e| Error::Database(e.into()))?;
        let mut tx = SqliteTx(native_tx);
        let result = f(&mut tx);
        match result {
            Ok(data) => {
                tx.0.commit().map_err(|e| Error::Database(e.into()))?;
                Ok(data)
            }
            Err(e) => {
                tx.0.rollback().map_err(|e| Error::Database(e.into()))?;
                Err(e)
            }
        }
    }
}

