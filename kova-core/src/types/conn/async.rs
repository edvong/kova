use crate::Result;
use crate::types::conn::Executable;
use crate::types::{Async, Connection};

pub trait ExecutableAsync: Executable<Mode = Async> {
    async fn with_native_async<F, R>(&self, f: F) -> Result<R>
    where
        F: AsyncFnOnce(&Self::Native) -> Result<R> + Send;
}

pub trait ConnectionAsync: Connection + ExecutableAsync {
    async fn transaction_async<F, R>(&self, f: F) -> Result<R>
    where
        F: AsyncFnOnce(&mut Self::Transaction<'_>) -> Result<R> + Send;
        // F: for<'tx> FnOnce(&'tx mut Self::Transaction<'_>) -> BoxFuture<'tx, Result<R>> + Send;
}

impl<E: ExecutableAsync> ExecutableAsync for &mut E {
    async fn with_native_async<F, R>(&self, f: F) -> Result<R>
    where
        F: AsyncFnOnce(&Self::Native) -> Result<R> + Send,
    {
        (**self).with_native_async(f).await
    }
}