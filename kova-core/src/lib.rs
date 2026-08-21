mod error;
pub mod types;
pub mod dsl;
pub mod render;
pub mod statement;
pub mod database;

pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;
// pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;





