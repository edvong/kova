use crate::query::Query;
use crate::types::conn::SqlitePool;
use crate::types::ext::SqliteExtension;
use crate::types::{Row, Value};
use kova_core::types::Database;

pub struct Sqlite;

impl Database for Sqlite {
    const MAX_PARAMS: usize = 32766;
    type Value<'a> = Value<'a>;
    type Row<'r> = Row<'r>;
    type Connection<'a> = SqlitePool;
    type Query<'a> = Query;
    type Extension<'e> = SqliteExtension;
}
