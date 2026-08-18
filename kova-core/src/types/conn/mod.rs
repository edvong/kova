mod sync;
mod r#async;

pub use self::sync::{ConnectionSync, ExecutableSync};
pub use self::r#async::{ConnectionAsync, ExecutableAsync};

pub trait Connection {
    type Transaction<'t> where Self: 't;
}
