use kova_core::types::{Async, Connection, ConnectionAsync, Executable, ExecutableAsync};
use kova_core::{Error, Result};

use deadpool_postgres::Transaction;
use deadpool_postgres::{GenericClient, Object, Pool};

pub struct PgPool(pub Pool);

pub struct PgTx<'t>(pub Transaction<'t>);

impl Executable for PgPool {
    type Mode = Async;
    type Native = Object;
}

impl ExecutableAsync for PgPool {
    async fn with_native_async<F, R>(&self, f: F) -> Result<R>
    where
        F: AsyncFnOnce(&Self::Native) -> Result<R> + Send
    {
        let guard = self.0.get().await.map_err(|e| Error::Database(e.into()))?;
        f(&guard).await
    }
}

impl<'t> Executable for PgTx<'t> {
    type Mode = Async;
    type Native = Transaction<'t>;
}

impl<'t> ExecutableAsync for PgTx<'t> {
    async fn with_native_async<F, R>(&self, f: F) -> Result<R>
    where
        F: AsyncFnOnce(&Self::Native) -> Result<R> + Send
    {
        // Borrow original transaction to execute query
        f(&self.0).await
    }
}

impl Connection for PgPool {
    type Transaction<'t> = PgTx<'t>;
}

impl ConnectionAsync for PgPool {
    async fn transaction_async<F, R>(&self, f: F) -> Result<R>
    where
        F: AsyncFnOnce(&mut Self::Transaction<'_>) -> Result<R> + Send
    {
        let mut guard = self.0.get().await.map_err(|e| Error::Database(e.into()))?;
        let native_tx = guard.transaction().await.map_err(|e| Error::Database(e.into()))?;
        let mut tx = PgTx(native_tx);
        let result = f(&mut tx).await;
        match result {
            Ok(data) => {
                tx.0.commit().await.map_err(|e| Error::Database(e.into()))?;
                Ok(data)
            }
            Err(e) => {
                tx.0.rollback().await.map_err(|e| Error::Database(e.into()))?;
                Err(e)
            }
        }
    }
}