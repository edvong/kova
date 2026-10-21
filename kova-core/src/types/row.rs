use crate::Result;
use crate::types::{Database, Decode};

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
    /// Extracts and decodes a column by trying `strict_name` (e.g., `"user.id"`) first,
    /// falling back to `col_name` (e.g., `"id"`).
    #[inline(always)]
    fn get_column<T, X>(&self, strict_name: &'static str, col_name: &'static str) -> Result<T>
        where
            DB: Decode<T, X> {
        let val = self.get_value(strict_name).or_else(|_| self.get_value(col_name))?;
        <DB as Decode<T, X>>::decode(val)
    }

    /// Retrieves a dynamically typed database value using either a zero-based index
    /// (`0_usize`) or a string-based column name (`"column_name"`).
    fn get_value<'i, I>(&self, idx: I) -> Result<DB::Value<'a>>
        where
            I: Into<ColumnIndex<'i>>;
}

#[diagnostic::on_unimplemented(
    message = "cannot extract `{Self}` from the database column(s)",
    label = "invalid column extraction target",
    note = "expected a type that implements `DecodeColumn` or `Decode`.\nif passing a tuple, ensure every element can be decoded from the database."
)]
pub trait DecodeColumn<DB: Database, X>: Sized {
    #[inline(always)]
    fn extract(row: &DB::Row<'_>) -> Result<Self> {
        let mut idx = 0;
        Self::extract_next(row, &mut idx)
    }

    fn extract_next(row: &DB::Row<'_>, idx: &mut usize) -> Result<Self>;

    #[inline(always)]
    fn extract_by<'a, F>(row: &DB::Row<'a>, idx: &mut usize, f: F) -> Result<Self>
        where
            F: FnOnce(DB::Value<'a>) -> Result<Self> {
        let val = Row::get_value(row, *idx)?;
        *idx += 1;
        f(val)
    }

    #[inline(always)]
    fn extract_value(row: &DB::Row<'_>, idx: &mut usize) -> Result<Self>
        where
            DB: Decode<Self, X> {
        Self::extract_by(row, idx, DB::decode)
    }
}

impl<DB, X, T> DecodeColumn<DB, X> for Option<T>
    where
        DB: Decode<Option<T>, X> {
    #[inline(always)]
    fn extract_next(row: &DB::Row<'_>, idx: &mut usize) -> Result<Self> {
        <Self as DecodeColumn<DB, X>>::extract_value(row, idx)
    }
}

impl<DB, X, T> DecodeColumn<DB, X> for Box<T>
    where
        DB: Decode<Box<T>, X> {
    #[inline(always)]
    fn extract_next(row: &DB::Row<'_>, idx: &mut usize) -> Result<Self> {
        <Self as DecodeColumn<DB, X>>::extract_value(row, idx)
    }
}

impl<DB, X, const N: usize> DecodeColumn<DB, X> for [u8; N]
    where
        DB: Decode<[u8; N], X> {
    #[inline(always)]
    fn extract_next(row: &DB::Row<'_>, idx: &mut usize) -> Result<Self> {
        <Self as DecodeColumn<DB, X>>::extract_value(row, idx)
    }
}

macro_rules! impl_from_column_for_tuple {
    ( $($T:ident),+ ) => {
        #[diagnostic::do_not_recommend]
        impl<DB: Database, X, $($T),+> DecodeColumn<DB, X> for ($($T,)+)
            where
                $( DB: Decode<$T, X> ),+ {
            #[inline(always)]
            fn extract_next(row: &DB::Row<'_>, idx: &mut usize) -> Result<Self> {
                Ok((
                    $(
                        {
                            let val = Row::get_value(row, *idx)?;
                            *idx += 1;
                            <DB as Decode<$T, X>>::decode(val)?
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
