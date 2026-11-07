use crate::query::Query;
use crate::types::conn::SqlitePool;
use crate::types::ext::SqliteExtension;
use crate::types::{Row, Value};
use kova_kernel::types::Database;

#[derive(Copy, Clone)]
pub struct Sqlite;

impl Database for Sqlite {
    const MAX_PARAMS: usize = 32766;
    type Value<'a> = Value<'a>;
    type Row<'a> = Row<'a>;
    type Connection = SqlitePool;
    type Query = Query;
    type Extension = SqliteExtension;
}
