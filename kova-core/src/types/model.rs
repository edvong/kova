use crate::types::{ColumnMeta, Database};
use crate::{Error, Result};

pub trait FromRow: Sized {

    type DB: Database;

    fn from_row(row: &<Self::DB as Database>::Row<'_>) -> Result<Self, Error>;

    fn from_row_as_values<'a>(row: &'a <Self::DB as Database>::Row<'_>)
        -> Result<Vec<<Self::DB as Database>::Value<'a>>, Error>;
}

pub trait Model: FromRow {

    const TABLE_NAME: &'static str;

    const COLUMNS_META: &'static [ColumnMeta];

    fn to_values<'a>(&'a self)
        -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>, Error>;
}