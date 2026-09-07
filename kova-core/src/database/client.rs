use crate::Result;
use crate::database::executor::Executor;
use crate::database::operations::Operations;
use crate::types::{Async, Connection, ConnectionAsync, ConnectionSync, Database, Mode, Sync};
use std::marker::PhantomData;
use std::ops::Deref;

pub struct Client<'a, DB: Database, M, X> {
    inner: Operations<'a, DB, DB::Connection<'a>, X>,
    _marker: PhantomData<M>,
}

impl<'a, DB: Database, M: Mode, X> Client<'a, DB, M, X> {
    pub fn new(conn: DB::Connection<'a>) -> Self {
        Self {
            inner: Operations::new(Executor::new(conn)),
            _marker: PhantomData,
        }
    }

    pub fn with<Codec>(self) -> Client<'a, DB, M, Codec> {
        Client {
            inner: self.inner.with::<Codec>(),
            _marker: self._marker,
        }
    }
}

impl<'a, DB: Database, M: Mode, X> Deref for Client<'a, DB, M, X> {
    type Target = Operations<'a, DB, DB::Connection<'a>, X>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// 'a is the connection pool,
// 'b is the mutable borrow itself,
// 't is the lifetime of the underlying native transaction
pub type Transaction<'a, 'b, 't, DB, X> = Operations<
    'a,
    DB,
    &'b mut <<DB as Database>::Connection<'a> as Connection>::Transaction<'t>,
    X,
>;

impl<'a, DB, X> Client<'a, DB, Sync, X>
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

impl<'a, DB, X> Client<'a, DB, Async, X>
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
