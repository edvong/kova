use crate::types::conn::SqlitePool;
use crate::types::ext::SqliteExtension;
use crate::types::{Row, Value};
use kova_core::types::{Sync, Database};

pub struct Sqlite;

impl Database for Sqlite {
    type Mode = Sync;
    type Connection<'a> = SqlitePool;
    type Value<'a> = Value<'a>;
    type Row<'r> = Row<'r>;
    type Extension<'e> = SqliteExtension;
}