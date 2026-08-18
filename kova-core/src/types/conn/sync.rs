use crate::Result;
use crate::types::Connection;

pub trait ExecutableSync: Sized {
    type Native;

    fn with_native_sync<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Self::Native) -> Result<R>;
}

pub trait ConnectionSync: Connection + ExecutableSync {
    fn transaction_sync<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self::Transaction<'_>) -> Result<R>;
}

impl<E: ExecutableSync> ExecutableSync for &mut E {
    type Native = E::Native;
    fn with_native_sync<F, R>(&self, f: F) -> Result<R> where F: FnOnce(&Self::Native) -> Result<R> {
        (**self).with_native_sync(f)
    }
}