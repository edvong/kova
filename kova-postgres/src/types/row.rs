use crate::types::Postgres;
use crate::types::value::Value;
use kova_core::Error;
use kova_core::types::{ColumnIndex, Row as CoreRow};

pub struct Row<'a> {
    pub(crate) inner: &'a tokio_postgres::Row,
}

impl<'a> Row<'a> {
    pub(crate) fn new(inner: &'a tokio_postgres::Row) -> Self {
        Self { inner }
    }
}

impl<'a> CoreRow<'a, Postgres> for Row<'a> {
    #[inline(always)]
    fn column_name(&self, idx: usize) -> Option<&str> {
        self.inner.columns().get(idx).map(|c| c.name())
    }

    fn get_value<'i, I>(&self, idx: I) -> kova_core::Result<Value<'a>>
        where
            I: Into<ColumnIndex<'i>> {
        match idx.into() {
            ColumnIndex::Index(i) => self.inner.try_get(i).map_err(Error::database),
            ColumnIndex::Name(n) => self.inner.try_get(n).map_err(Error::database),
        }
    }
}
