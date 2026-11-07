use crate::Result;
use crate::database::executor::Executor;
use crate::database::operations::Operations;
use crate::types::{Async, Connection, ConnectionAsync, ConnectionSync, Database, Mode, Sync};
use std::marker::PhantomData;
use std::ops::Deref;

pub struct Client<DB: Database, M, X> {
    inner: Operations<DB, DB::Connection, X>,
    _marker: PhantomData<M>,
}

// Manual Clone implementation so `X` (the user's Codec marker) is not required to implement `Clone`.
impl<DB: Database, M, X> Clone for Client<DB, M, X>
    where
        DB::Connection: Clone {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone(), _marker: PhantomData }
    }
}

impl<DB: Database, M: Mode, X> Client<DB, M, X> {
    pub fn new(conn: DB::Connection) -> Self {
        Self { inner: Operations::new(Executor::new(conn)), _marker: PhantomData }
    }

    pub fn codec<CX>(self) -> Client<DB, M, CX> {
        Client { inner: self.inner.codec::<CX>(), _marker: self._marker }
    }
}

impl<DB: Database, M: Mode, X> Deref for Client<DB, M, X> {
    type Target = Operations<DB, DB::Connection, X>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// 'b is the mutable borrow itself,
// 't is the lifetime of the underlying native transaction.
pub type Transaction<'b, 't, DB, X> =
    Operations<DB, &'b mut <<DB as Database>::Connection as Connection>::Transaction<'t>, X>;

impl<DB, X> Client<DB, Sync, X>
    where
        DB: Database,
        DB::Connection: ConnectionSync {
    pub fn transaction<F, R>(&self, f: F) -> Result<R>
        where
            F: FnOnce(Transaction<'_, '_, DB, X>) -> Result<R> {
        self.executor
            .inner
            .transaction_sync(|native_tx| f(Operations::new(Executor::new(native_tx))))
    }
}

impl<DB, X> Client<DB, Async, X>
    where
        DB: Database,
        DB::Connection: ConnectionAsync {
    pub async fn transaction<F, R>(&self, f: F) -> Result<R>
        where
            F: AsyncFnOnce(Transaction<'_, '_, DB, X>) -> Result<R> + Send {
        self.executor
            .inner
            .transaction_async(async |native_tx| f(Operations::new(Executor::new(native_tx))).await)
            .await
    }
}
