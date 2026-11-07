#![allow(async_fn_in_trait)]

mod r#async;
mod sync;

pub use self::r#async::{ConnectionAsync, ExecutableAsync};
pub use self::sync::{ConnectionSync, ExecutableSync};
use crate::types::Mode;

pub trait Executable: Sized {
    type Mode: Mode;
    type Native;
}

pub trait Connection {
    type Transaction<'t>
        where
            Self: 't;
}

impl<E: Executable> Executable for &mut E {
    type Mode = E::Mode;
    type Native = E::Native;
}
