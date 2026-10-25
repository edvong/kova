use crate::types::Sqlite;
use crate::types::value::Value;
use kova_core::Error;
use kova_core::types::{ColumnIndex, Row as CoreRow};

pub struct Row<'a> {
    pub(crate) inner: &'a rusqlite::Row<'a>,
}

impl<'a> Row<'a> {
    pub(crate) fn new(inner: &'a rusqlite::Row<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> CoreRow<'a, Sqlite> for Row<'a> {
    #[inline(always)]
    fn column_name(&self, idx: usize) -> Option<&str> {
        self.inner.as_ref().column_name(idx).ok()
    }

    fn get_value<'i, I>(&self, idx: I) -> kova_core::Result<Value<'a>>
        where
            I: Into<ColumnIndex<'i>> {
        let val_ref = match idx.into() {
            ColumnIndex::Index(i) => self.inner.get_ref(i),
            ColumnIndex::Name(n) => self.inner.get_ref(n),
        }
        .map_err(Error::database)?;

        Value::from_value_ref(val_ref)
    }
}

