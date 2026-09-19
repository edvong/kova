mod models;
mod compile_check_select;
mod compile_check_insert;
mod compile_check_update;
mod compile_check_delete;
mod compile_check_json;
mod codec;
mod view;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;