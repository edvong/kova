use crate::types::Sqlite;
use crate::types::value::Value;
use kova_core::types::{ColumnIndex, Row as CoreRow};
use kova_core::Error;

/// A lightweight, zero-copy wrapper around `rusqlite::Row`.
///
/// This struct bridges the native rusqlite row representation with the ORM's
/// core generic abstraction. It explicitly bounds data extraction to the statement's
/// lifetime `'a`, enabling safe, zero-allocation reads directly from SQLite's C buffers.
pub struct Row<'a> {
    pub(crate) inner: &'a rusqlite::Row<'a>,
}

impl<'a> Row<'a> {
    /// Internal constructor to wrap the native driver row.
    pub(crate) fn new(inner: &'a rusqlite::Row<'a>) -> Self {
        Self { inner }
    }
}

/// Implements the core generic row data access interface for SQLite.
impl<'a> CoreRow<'a, Sqlite> for Row<'a> {
    /// Retrieves a raw `Value` from the row using either a numeric index or a string-based column name.
    ///
    /// Crucially, this leverages `get_ref()` to obtain a `ValueRef`, preserving
    /// exact zero-copy semantics before wrapping it into the ORM's unified `Value` enum.
    fn get_value<'i, I>(&self, idx: I) -> kova_core::Result<Value<'a>>
        where
            I: Into<ColumnIndex<'i>> {
        let val_ref = match idx.into() {
            ColumnIndex::Index(i) => self.inner.get_ref(i),
            ColumnIndex::Name(n) => self.inner.get_ref(n),
        }
        .map_err(|e| Error::Database(Box::new(e)))?;

        Value::from_value_ref(val_ref)
    }
}

