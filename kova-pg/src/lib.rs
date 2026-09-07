pub mod types;
pub mod query;
pub mod dsl;
pub mod render;
mod options;

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Features 'rustls' and 'native-tls' are mutually exclusive.");