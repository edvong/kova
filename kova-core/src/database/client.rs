use crate::Result;
use crate::database::executor::Executor;
use crate::database::operations::Operations;
use crate::types::{Sync, Connection, ConnectionAsync, ConnectionSync, Database, Mode, Async};
use std::marker::PhantomData;
use std::ops::Deref;

pub struct Client<'a, DB: Database, M: Mode = <DB as Database>::Mode> {
    pub(crate) inner: Operations<DB::Connection<'a>>,
    pub(crate) _mode: PhantomData<M>,
}

impl<'a, DB: Database, M: Mode> Deref for Client<'a, DB, M> {
    type Target = Operations<DB::Connection<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// 'a is the connection pool,
// 'b is the mutable borrow itself,
// 't is the lifetime of the underlying native transaction
pub type Transaction<'a, 'b, 't, DB> = Operations<&'b mut <<DB as Database>::Connection<'a> as Connection>::Transaction<'t>>;

impl<'a, DB> Client<'a, DB, Sync>
where
    DB: Database<Mode =Sync>,
    DB::Connection<'a>: ConnectionSync,
{
    pub fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(Transaction<'a, '_, '_, DB>) -> Result<R> {
        self.executor.inner.transaction_sync(|native_tx| {
            f(Operations::new(Executor::new(native_tx)))
        })
    }
}


impl<'a, DB> Client<'a, DB, Async>
where
    DB: Database<Mode =Async>,
    DB::Connection<'a>: ConnectionAsync,
{
    pub async fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: AsyncFnOnce(Transaction<'a, '_, '_, DB>) -> Result<R> + Send
    {
        self.executor.inner.transaction_async(async |native_tx| {
            f(Operations::new(Executor::new(native_tx))).await
        }).await
    }
}