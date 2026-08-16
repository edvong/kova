use crate::types::value::Value;
use kova_core::Error;
use std::fmt::Display;
use tokio_postgres::row::RowIndex;

pub struct Row<'a> {
    pub(crate) inner: &'a tokio_postgres::Row,
}

impl<'a> Row<'a> {
    pub(crate) fn new(inner: &'a tokio_postgres::Row) -> Self {
        Self { inner }
    }

    // Macro-oriented code generation: Uniformly extracted as Kova's Value enumeration, zero-copy for the whole link.
    pub fn get_value<I: RowIndex + Display>(&self, idx: I) -> kova_core::Result<Value<'a>> {
        // Since `inner` is `&'a Row`, `try_get` perfectly matches and returns a `Value<'a>` bound to `'a`
        self.inner
            .try_get::<I, Value<'a>>(idx)
            .map_err(|e| Error::Database(Box::new(e)))
    }
}