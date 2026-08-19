use crate::types::conn::PgPool;
use crate::types::ext::PgExtension;
use crate::types::{Row, Value};
use kova_core::types::Database;

pub struct Postgres;

impl Database for Postgres {
    type Connection<'a> = PgPool;
    type Value<'a> = Value<'a>;
    type Row<'r> = Row<'r>;
    type Extension<'e> = PgExtension;
}