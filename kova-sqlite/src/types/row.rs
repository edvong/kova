use crate::types::Sqlite;
use crate::types::value::Value;
use kova_core::Error;
use kova_core::types::{ColumnIndex, FromValue, GetColumn, Row as CoreRow};

pub struct Row<'a> {
    pub(crate) inner: &'a rusqlite::Row<'a>,
}

impl<'a> Row<'a> {
    pub(crate) fn new(inner: &'a rusqlite::Row<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> CoreRow<'a, Sqlite> for Row<'a> {
    fn get_value<'i, I>(&self, idx: I) -> kova_core::Result<Value<'a>>
    where
        I: Into<ColumnIndex<'i>>,
    {
        let val_ref = match idx.into() {
            ColumnIndex::Index(i) => self.inner.get_ref(i),
            ColumnIndex::Name(n) => self.inner.get_ref(n),
        }
        .map_err(|e| Error::Database(Box::new(e)))?;

        Value::from_value_ref(val_ref)
    }
}

impl<'a, T: FromValue<Sqlite>> GetColumn<'a, T> for Row<'a> {
    fn get_column(&self, idx: usize) -> kova_core::Result<T> {
        let value = self
            .get_value(idx)
            .map_err(|e| Error::Database(Box::new(e)))?;
        T::from_value(value)
    }
}

kova_core::bind_get_columns_tuple!(Sqlite, Row);
