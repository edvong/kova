use crate::Result;
use crate::database::executor::Executor;
use crate::database::operations::Operations;
use crate::types::{Async, Connection, ConnectionAsync, ConnectionSync, Database, Executable, Mode, Sync};
use std::marker::PhantomData;
use std::ops::Deref;

type EM<'a, DB> = <<DB as Database>::Connection<'a> as Executable>::Mode;

pub struct Client<'a, DB: Database, M: Mode = EM<'a, DB>> {
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
    DB::Connection<'a>: ConnectionSync,
    DB: Database
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
    DB::Connection<'a>: ConnectionAsync,
    DB: Database
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