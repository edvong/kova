use crate::query::Query;
use crate::types::conn::PgPool;
use crate::types::ext::Extension;
use crate::types::{Row, Value};
use kova_core::types::Database;

#[derive(Copy, Clone)]
pub struct Postgres;

impl Database for Postgres {
    const MAX_PARAMS: usize = 65535;
    type Value<'a> = Value<'a>;
    type Row<'a> = Row<'a>;
    type Connection = PgPool;
    type Query = Query;
    type Extension = Extension;
}
