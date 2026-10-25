use crate::Result;
use crate::types::{ExecutableAsync, ExecutableSync};

pub struct Executor<E> {
    pub(crate) inner: E,
}

impl<E: Clone> Clone for Executor<E> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<'a, E> Executor<E> {
    pub(crate) fn new(e: E) -> Self {
        Self { inner: e }
    }

    #[inline(always)]
    pub fn inner(&self) -> &E {
        &self.inner
    }
}

impl<'a, E: ExecutableSync> Executor<E> {
    pub fn with_conn_sync<F, R>(&self, f: F) -> Result<R>
        where
            F: FnOnce(&E::Native) -> Result<R> {
        self.inner.with_native_sync(f)
    }
}

impl<'a, E: ExecutableAsync> Executor<E> {
    pub async fn with_conn_async<F, R>(&self, f: F) -> Result<R>
        where
            F: AsyncFnOnce(&E::Native) -> Result<R> + Send {
        self.inner.with_native_async(f).await
    }
}
