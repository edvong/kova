mod models;
mod compile_check_select;
mod compile_check_insert;
mod compile_check_update;
mod compile_check_delete;
mod compile_check_json;
mod codec;
mod view;
mod compile_check_fts5;
mod compile_check_advanced;
mod compile_check_tag;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;