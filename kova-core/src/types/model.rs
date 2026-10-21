use crate::types::{ColumnMeta, Database};
use crate::{Error, Result};

pub trait Relation: Sized {
    const NAME: &'static str;
}

pub trait Cte: Relation { }

#[diagnostic::on_unimplemented(
    message = "cannot map a database row to `{Self}` (under codec `{X}`)",
    label = "invalid row mapping target",
    note = "expected a type that implements `FromRow<{DB}, {X}>`.\nif `{Self}` is a struct, ensure it derives `Model` or `FromRow` and all fields implement `Decode` under codec `{X}`."
)]
pub trait FromRow<DB: Database, X>: Sized {
    fn from_row(row: &DB::Row<'_>) -> Result<Self>;
}

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a valid `Model` under codec `{X}`",
    label = "invalid table model for this codec",
    note = "ensure `{Self}` derives `Model` and all of its fields implement `Encode` and `Decode` under codec `{X}`."
)]
pub trait Model<DB: Database, X>: FromRow<DB, X> + Relation {
    const COLUMNS_META: &'static [ColumnMeta];

    fn to_values<'a>(&'a self) -> Result<Vec<(&'static str, DB::Value<'a>)>>;
}

impl<T, DB, X> FromRow<DB, X> for Option<T>
    where
        DB: Database,
        T: FromRow<DB, X> {
    fn from_row(row: &DB::Row<'_>) -> Result<Self> {
        match T::from_row(row) {
            Ok(val) => Ok(Some(val)),
            Err(Error::UnexpectedNull(_)) | Err(Error::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

macro_rules! impl_from_row_for_tuple {
    ( $($T:ident),+ ) => {
        #[diagnostic::do_not_recommend]
        impl<$($T,)+ DB, X> FromRow<DB, X> for ($($T,)+)
            where
                DB: Database,
                $( $T: FromRow<DB, X>, )+ {
            fn from_row(row: &DB::Row<'_>) -> Result<Self> {
                Ok(( $( $T::from_row(row)?, )+ ))
            }
        }
    };
}

#[rustfmt::skip]
mod macros {
    use super::*;
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
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    impl_from_row_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32);
}