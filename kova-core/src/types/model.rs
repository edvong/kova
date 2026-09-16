use crate::types::{ColumnMeta, Database};
use crate::{Error, Result};

#[diagnostic::on_unimplemented(
    message = "cannot map a database row to `{Self}`",
    label = "invalid row mapping target",
    note = "expected a type that implements `FromRow`.\nif `{Self}` is a struct, ensure it derives `Model` or `FromRow`.\nif passing a tuple, ensure every element implements `FromRow`."
)]
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
        #[diagnostic::do_not_recommend]
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

impl_from_row_for_tuple!(T1);
impl_from_row_for_tuple!(T1, T2);
impl_from_row_for_tuple!(T1, T2, T3);
impl_from_row_for_tuple!(T1, T2, T3, T4);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
);
impl_from_row_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32
);
