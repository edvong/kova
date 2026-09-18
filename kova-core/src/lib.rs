pub mod database;
pub mod dsl;
mod error;
pub mod render;
pub mod types;

pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;
// pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;
