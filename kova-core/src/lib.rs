mod error;
mod type_id;
pub mod database;
pub mod dsl;
pub mod render;
pub mod types;

pub use type_id::type_id;
pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;
// pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;
