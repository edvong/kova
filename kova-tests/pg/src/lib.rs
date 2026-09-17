mod models;
mod compile_check_select;
mod compile_check_insert;
mod compile_check_update;
mod compile_check_delete;
mod codec;
mod view;
mod compile_check_operator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;