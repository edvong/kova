use crate::Result;
use crate::types::{Database, Decode};

/// A unified column index enumeration.
///
/// Decouples the core library's row indexing logic from specific underlying database driver
/// implementations, allowing seamless data retrieval by either numeric index or string-based column name.
pub enum ColumnIndex<'i> {
    Index(usize),
    Name(&'i str),
}

impl From<usize> for ColumnIndex<'_> {
    fn from(idx: usize) -> Self {
        ColumnIndex::Index(idx)
    }
}

impl<'i> From<&'i str> for ColumnIndex<'i> {
    fn from(name: &'i str) -> Self {
        ColumnIndex::Name(name)
    }
}

/// Core protocol for extracting raw values from a database row.
pub trait Row<'a, DB: Database> {
    /// Retrieves a dynamically typed database value using either a zero-based index
    /// (`0_usize`) or a string-based column name (`"column_name"`).
    fn get_value<'i, I>(&self, idx: I) -> Result<DB::Value<'a>>
    where
        I: Into<ColumnIndex<'i>>;
}

pub trait FromColumn<'a, DB: Database, X>: Sized {
    #[inline(always)]
    fn extract(row: &DB::Row<'a>) -> Result<Self> {
        let mut idx = 0;
        Self::extract_next(row, &mut idx)
    }

    fn extract_next(row: &DB::Row<'a>, idx: &mut usize) -> Result<Self>;

    #[inline(always)]
    fn extract_by<F>(row: &DB::Row<'a>, idx: &mut usize, f: F) -> Result<Self>
    where
        F: FnOnce(DB::Value<'a>) -> Result<Self>,
    {
        let val = Row::get_value(row, *idx)?;
        *idx += 1;
        f(val)
    }

    #[inline(always)]
    fn extract_value(row: &DB::Row<'a>, idx: &mut usize) -> Result<Self>
    where
        Self: Decode<'a, DB, X>,
    {
        Self::extract_by(row, idx, Self::decode)
    }
}
