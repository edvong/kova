extern crate core;

pub mod type_check;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
