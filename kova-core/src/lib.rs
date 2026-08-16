mod error;
pub mod types;
pub mod dsl;
pub mod render;

pub type Result<T, E = crate::error::Error> = std::result::Result<T, E>;
pub use error::Error;





