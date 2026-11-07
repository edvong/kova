mod error;
pub mod database;
pub mod dsl;
pub mod render;
pub mod types;

pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;
