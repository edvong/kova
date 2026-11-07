use crate::types::Value;
use deadpool_postgres::{GenericClient, Object, Pool, Transaction};
use kova_kernel::types::{Async, Connection, ConnectionAsync, Executable, ExecutableAsync};
use kova_kernel::{Error, Result};
use tokio_postgres::Statement;
use tokio_postgres::types::{ToSql, Type};

pub type Inspector = fn(&str, &[Value<'_>]);

#[derive(Clone)]
pub struct PgPool {
    pub(crate) pool: Pool,
    pub(crate) inspector: Option<Inspector>,
}

pub struct PgTx<'t> {
    pub(crate) tx: Transaction<'t>,
    pub(crate) inspector: Option<Inspector>,
}

#[allow(async_fn_in_trait)]
pub trait PgClient: GenericClient + Sync {
    async fn prepare_cached(&self, sql: &str, types: &[Type]) -> Result<Statement, tokio_postgres::Error>;

    #[inline]
    async fn execute_cached<'a>(&self, sql: &str, params: &[Value<'a>]) -> Result<u64> {
        let types: Vec<Type> = params.iter().map(Value::param_type).collect();
        let stmt = <Self as PgClient>::prepare_cached(self, sql, &types).await
            .map_err(Error::database)?;
        self.execute_raw(&stmt, params).await.map_err(Error::database)
    }

    #[inline]
    async fn query_cached<'a>(&self, sql: &str, params: &[Value<'a>]) -> Result<Vec<tokio_postgres::Row>> {
        let mut param_refs: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(params.len());
        let mut types: Vec<Type> = Vec::with_capacity(params.len());
        for param in params {
            param_refs.push(param as &(dyn ToSql + Sync));
            types.push(param.param_type());
        }
        let stmt = <Self as PgClient>::prepare_cached(self, sql, &types).await
            .map_err(Error::database)?;
        self.query(&stmt, &param_refs).await.map_err(Error::database)
    }
}

impl PgClient for Object {
    #[inline(always)]
    async fn prepare_cached(&self, sql: &str, types: &[Type]) -> Result<Statement, tokio_postgres::Error> {
        self.prepare_typed_cached(sql, types).await
    }
}

impl PgClient for Transaction<'_> {
    #[inline(always)]
    async fn prepare_cached(&self, sql: &str, types: &[Type]) -> Result<Statement, tokio_postgres::Error> {
        self.prepare_typed_cached(sql, types).await
    }
}

pub trait PgExecutor: ExecutableAsync<Native: PgClient> + Sync {
    fn inspect(&self, sql: &str, params: &[Value<'_>]);
}

impl PgExecutor for PgPool {
    #[inline(always)]
    fn inspect(&self, sql: &str, params: &[Value<'_>]) {
        if let Some(f) = self.inspector {
            f(sql, params);
        }
    }
}

impl PgExecutor for &mut PgTx<'_> {
    #[inline(always)]
    fn inspect(&self, sql: &str, params: &[Value<'_>]) {
        if let Some(f) = self.inspector {
            f(sql, params);
        }
    }
}

impl Executable for PgPool {
    type Mode = Async;
    type Native = Object;
}

impl ExecutableAsync for PgPool {
    async fn with_native_async<F, R>(&self, f: F) -> Result<R>
        where
            F: AsyncFnOnce(&Self::Native) -> Result<R> + Send {
        let guard = self.pool.get().await.map_err(Error::database)?;
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
            F: AsyncFnOnce(&Self::Native) -> Result<R> + Send {
        f(&self.tx).await
    }
}

impl Connection for PgPool {
    type Transaction<'t> = PgTx<'t>;
}

impl ConnectionAsync for PgPool {
    async fn transaction_async<F, R>(&self, f: F) -> Result<R>
        where
            F: AsyncFnOnce(&mut Self::Transaction<'_>) -> Result<R> + Send {
        let mut guard = self.pool.get().await.map_err(Error::database)?;
        let native_tx = guard.transaction().await.map_err(Error::database)?;
        let mut tx = PgTx { tx: native_tx, inspector: self.inspector };
        let result = f(&mut tx).await;
        match result {
            Ok(data) => {
                tx.tx.commit().await.map_err(Error::database)?;
                Ok(data)
            }
            Err(e) => {
                tx.tx.rollback().await.map_err(Error::database)?;
                Err(e)
            }
        }
    }
}