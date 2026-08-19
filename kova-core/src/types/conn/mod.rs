#![allow(async_fn_in_trait)]

mod sync;
mod r#async;

use crate::types::Mode;
pub use self::sync::{ConnectionSync, ExecutableSync};
pub use self::r#async::{ConnectionAsync, ExecutableAsync};

pub trait Executable: Sized {
    type Mode: Mode;
    type Native;
}

pub trait Connection {
    type Transaction<'t> where Self: 't;
}

impl<E: Executable> Executable for &mut E {
    type Mode = E::Mode;
    type Native = E::Native;
}
