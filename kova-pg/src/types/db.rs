use crate::query::Query;
use crate::types::conn::PgPool;
use crate::types::ext::Extension;
use crate::types::{Row, Value};
use kova_core::types::Database;

pub struct Postgres;

impl Database for Postgres {
    type Value<'a> = Value<'a>;
    type Row<'r> = Row<'r>;
    type Connection<'a> = PgPool;
    type Query<'a> = Query;
    type Extension<'e> = Extension;
}