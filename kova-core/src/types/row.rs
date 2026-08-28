use crate::Result;
use crate::types::Database;

/// Unified column index enumeration,
/// cutting off the core library's RowIndex dependency on the underlying driver
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

/// Row extracts protocol
pub trait Row<'a, DB: Database> {
    /// Get Value by index or name, support 0_usize or "column_name".
    fn get_value<'i, I>(&self, idx: I) -> Result<DB::Value<'a>>
    where
        I: Into<ColumnIndex<'i>>;
}

pub trait GetColumn<'a, Target> {
    fn get_column(&self, idx: usize) -> Result<Target>;
}

pub trait GetColumns<'a, Target> {
    fn get_columns(&self, idx: &mut usize) -> Result<Target>;
}

#[macro_export]
#[doc(hidden)]
macro_rules! bind_get_columns_tuple_inner {
    ($DB:ty, $Row:ident, $($T:ident),+) => {
        impl<'a, $($T),+> $crate::types::GetColumns<'a, ($($T,)+)> for $Row<'a>
        where
            // Each item of the constraint tuple can be converted from a Value in the database.
            $($T: $crate::types::FromValue<$DB>),+
        {
            fn get_columns(&self, idx: &mut usize) -> $crate::Result<($($T,)+)> {
                Ok((
                    $(
                        {
                            let val = $crate::types::GetColumn::<'a, $T>::get_column(self, *idx)?;
                            *idx += 1;
                            val
                        },
                    )+
                ))
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! bind_get_columns_tuple {
    ($DB:ty, $Row:ident) => {
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2, T3);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2, T3, T4);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2, T3, T4, T5);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2, T3, T4, T5, T6);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2, T3, T4, T5, T6, T7);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $crate::bind_get_columns_tuple_inner!($DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24, T25
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24, T25, T26
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24, T25, T26, T27
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
        );
        $crate::bind_get_columns_tuple_inner!(
            $DB, $Row, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32
        );
    };
}
