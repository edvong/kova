use crate::types::ext::PgExtension;
use crate::types::{Row, Value};
use kova_core::types::Database;

pub struct Postgres;

impl Database for Postgres {
    type Value<'a> = Value<'a>;
    type Row<'r> = Row<'r>;
    type Extension<'e> = PgExtension;
}