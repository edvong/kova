use crate::types::Postgres;
use crate::types::value::Value;
use kova_core::types::{ColumnIndex, Row as CoreRow};
use kova_core::Error;

/// A lightweight, zero-copy wrapper around `tokio_postgres::Row`.
///
/// This struct bridges the native tokio-postgres row representation with the ORM's
/// core generic abstraction. It ensures that data extraction strictly adheres to
/// the underlying network buffer's lifetime `'a`, preventing unnecessary memory allocations.
pub struct Row<'a> {
    pub(crate) inner: &'a tokio_postgres::Row,
}

impl<'a> Row<'a> {
    /// Internal constructor to wrap the native driver row.
    pub(crate) fn new(inner: &'a tokio_postgres::Row) -> Self {
        Self { inner }
    }
}

/// Implements the core generic row data access interface for PostgreSQL.
impl<'a> CoreRow<'a, Postgres> for Row<'a> {
    /// Retrieves a raw `Value` from the row using either a zero-based numeric index
    /// or a string-based column name.
    /// Driver-level retrieval errors are safely caught and boxed into the uniform `kova_core::Error`.
    fn get_value<'i, I>(&self, idx: I) -> kova_core::Result<Value<'a>>
        where
            I: Into<ColumnIndex<'i>> {
        match idx.into() {
            ColumnIndex::Index(i) => {
                self.inner.try_get(i).map_err(|e| Error::Database(Box::new(e)))
            }
            ColumnIndex::Name(n) => self.inner.try_get(n).map_err(|e| Error::Database(Box::new(e))),
        }
    }
}
