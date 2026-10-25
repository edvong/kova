pub mod type_check;

#[cfg(test)]
pub mod render;
#[cfg(test)]
pub mod runtime;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
