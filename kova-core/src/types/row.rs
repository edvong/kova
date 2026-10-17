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

#[diagnostic::on_unimplemented(
    message = "cannot extract `{Self}` from the database column(s)",
    label = "invalid column extraction target",
    note = "expected a type that implements `FromColumn` or `Decode`.\nif passing a tuple, ensure every element can be decoded from the database."
)]
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
            F: FnOnce(DB::Value<'a>) -> Result<Self> {
        let val = Row::get_value(row, *idx)?;
        *idx += 1;
        f(val)
    }

    #[inline(always)]
    fn extract_value(row: &DB::Row<'a>, idx: &mut usize) -> Result<Self>
        where
            DB: Decode<'a, Self, X> {
        Self::extract_by(row, idx, DB::decode)
    }
}

impl<'a, DB, X, T> FromColumn<'a, DB, X> for Option<T>
    where
        DB: Decode<'a, Option<T>, X> {
    #[inline(always)]
    fn extract_next(row: &DB::Row<'a>, idx: &mut usize) -> Result<Self> {
        <Self as FromColumn::<DB, X>>::extract_value(row, idx)
    }
}

impl<'a, DB, X, T> FromColumn<'a, DB, X> for Box<T>
    where
        DB: Decode<'a, Box<T>, X> {
    #[inline(always)]
    fn extract_next(row: &DB::Row<'a>, idx: &mut usize) -> Result<Self> {
        <Self as FromColumn::<DB, X>>::extract_value(row, idx)
    }
}

impl<'a, DB, X, const N: usize> FromColumn<'a, DB, X> for [u8; N]
    where
        DB: Decode<'a, [u8; N], X> {
    #[inline(always)]
    fn extract_next(row: &DB::Row<'a>, idx: &mut usize) -> Result<Self> {
        <Self as FromColumn::<DB, X>>::extract_value(row, idx)
    }
}

macro_rules! impl_from_column_for_tuple {
    ( $($T:ident),+ ) => {
        #[diagnostic::do_not_recommend]
        impl<'a, DB: Database, X, $($T),+> FromColumn<'a, DB, X> for ($($T,)+)
        where
            $( DB: Decode<'a, $T, X> ),+
        {
            #[inline(always)]
            fn extract_next(row: &DB::Row<'a>, idx: &mut usize) -> Result<Self> {
                Ok((
                    $(
                        {
                            let val = Row::get_value(row, *idx)?;
                            *idx += 1;
                            <DB as Decode<'a, $T, X>>::decode(val)?
                        },
                    )+
                ))
            }
        }
    };
}

#[rustfmt::skip]
pub mod macros {
    use super::*;
impl_from_column_for_tuple!(T1);
impl_from_column_for_tuple!(T1, T2);
impl_from_column_for_tuple!(T1, T2, T3);
impl_from_column_for_tuple!(T1, T2, T3, T4);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
impl_from_column_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32);
}