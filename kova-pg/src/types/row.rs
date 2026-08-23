use crate::types::Postgres;
use crate::types::value::Value;
use kova_core::Error;
use kova_core::types::{ColumnIndex, FromValue, GetColumn, Row as CoreRow};

pub struct Row<'a> {
    pub(crate) inner: &'a tokio_postgres::Row,
}

impl<'a> Row<'a> {
    pub(crate) fn new(inner: &'a tokio_postgres::Row) -> Self {
        Self { inner }
    }
}

impl<'a> CoreRow<'a, Postgres> for Row<'a> {
    fn get_value<'i, I>(&self, idx: I) -> kova_core::Result<Value<'a>>
    where
        I: Into<ColumnIndex<'i>>,
    {
        match idx.into() {
            ColumnIndex::Index(i) => self
                .inner
                .try_get(i)
                .map_err(|e| Error::Database(Box::new(e))),
            ColumnIndex::Name(n) => self
                .inner
                .try_get(n)
                .map_err(|e| Error::Database(Box::new(e))),
        }
    }
}

impl<'a, T: FromValue<Postgres>> GetColumn<'a, T> for Row<'a> {
    fn get_column(&self, idx: usize) -> kova_core::Result<T> {
        let value = self.get_value(idx).map_err(|e| Error::Database(Box::new(e)))?;
        T::from_value(value)
    }
}

