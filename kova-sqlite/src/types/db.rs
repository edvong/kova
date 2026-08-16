use crate::types::{Row, Value};
use kova_core::types::Database;
use crate::types::ext::SqliteExtension;

pub struct Sqlite;

impl Database for Sqlite {
    type Value<'a> = Value<'a>;
    type Row<'r> = Row<'r>;
    type Extension<'e> = SqliteExtension;
}