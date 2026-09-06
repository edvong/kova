use crate::Result;
use crate::database::executor::Executor;
use crate::database::operations::Operations;
use crate::types::{
    Async, Connection, ConnectionAsync, ConnectionSync, Database, Executable, Mode, Sync,
};
use std::marker::PhantomData;
use std::ops::Deref;

type EM<'a, DB> = <<DB as Database>::Connection<'a> as Executable>::Mode;

pub struct Client<'a, DB: Database, X, M: Mode = EM<'a, DB>> {
    pub(crate) inner: Operations<'a, DB, DB::Connection<'a>, X>,
    pub(crate) _mode: PhantomData<M>,
}

impl<'a, DB: Database, X, M: Mode> Deref for Client<'a, DB, X, M> {
    type Target = Operations<'a, DB, DB::Connection<'a>, X>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Provider new with default `X` type & Specify `X` type

// 'a is the connection pool,
// 'b is the mutable borrow itself,
// 't is the lifetime of the underlying native transaction
pub type Transaction<'a, 'b, 't, DB, X> = Operations<
    'a,
    DB,
    &'b mut <<DB as Database>::Connection<'a> as Connection>::Transaction<'t>,
    X,
>;

impl<'a, DB, X> Client<'a, DB, X, Sync>
where
    DB::Connection<'a>: ConnectionSync,
    DB: Database,
    X: 'a,
{
    pub fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(Transaction<'a, '_, '_, DB, X>) -> Result<R>,
    {
        self.executor
            .inner
            .transaction_sync(|native_tx| f(Operations::new(Executor::new(native_tx))))
    }
}

impl<'a, DB, X> Client<'a, DB, X, Async>
where
    DB::Connection<'a>: ConnectionAsync,
    DB: Database,
    X: 'a,
{
    pub async fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: AsyncFnOnce(Transaction<'a, '_, '_, DB, X>) -> Result<R> + Send,
    {
        self.executor
            .inner
            .transaction_async(async |native_tx| f(Operations::new(Executor::new(native_tx))).await)
            .await
    }
}
