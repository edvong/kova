use crate::types::{ColumnMeta, Database};
use crate::{Error, Result};

pub trait FromRow: Sized {
    type DB: Database;

    fn from_row(row: &<Self::DB as Database>::Row<'_>) -> Result<Self, Error>;
}

pub trait Model: FromRow {
    const TABLE_NAME: &'static str;

    const COLUMNS_META: &'static [ColumnMeta];

    fn to_values<'a>(
        &'a self,
    ) -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>, Error>;
}

impl<T, DB> FromRow for Option<T>
where
    DB: Database,
    T: FromRow<DB = DB>,
{
    type DB = DB;
    fn from_row(row: &<DB as Database>::Row<'_>) -> Result<Self, Error> {
        match T::from_row(row) {
            Ok(val) => Ok(Some(val)),
            Err(Error::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

// Macros already implemented for single type
macro_rules! impl_from_row_for_tuple {
    ( $($T:ident),+ ) => {
        impl<$($T,)+ DB> FromRow for ($($T,)+)
        where
            DB: Database,
            $( $T: FromRow<DB = DB>, )+
        {
            type DB = DB;
            fn from_row(row: &<DB as Database>::Row<'_>) -> Result<Self, Error> {
                Ok((
                    $( $T::from_row(row)?, )+
                ))
            }
        }
    };
}

// impl_from_row_for_tuple!(T1);
impl_from_row_for_tuple!(T1, T2);
impl_from_row_for_tuple!(T1, T2, T3);
impl_from_row_for_tuple!(T1, T2, T3, T4);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
