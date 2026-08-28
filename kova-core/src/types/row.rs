use crate::Result;
use crate::types::{Database, FromValue};

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

/// Protocol for extracting a strongly-typed value from a specific column index.
pub trait GetColumn<'a, Target> {
    fn get_column(&self, idx: usize) -> Result<Target>;
}

/// Marker trait used to distinguish primitive scalar types from complex structures (like tuples).
///
/// This acts as a compiler firewall to prevent overlapping implementations
/// when generating row-to-tuple generic decoders.
pub trait ScalarMarker<DB: Database> {}

impl<DB, T> ScalarMarker<DB> for Option<T>
where
    DB: Database,
    T: ScalarMarker<DB>,
{}

impl<DB, T> ScalarMarker<DB> for Box<T>
where
    DB: Database,
    T: ScalarMarker<DB>,
{}

/// A convenience macro to quickly implement `ScalarMarker` for multiple standard types.
#[macro_export]
macro_rules! mark_scalars {
    ($DB:ty, $($T:ty),+) => {
        $(
            impl $crate::types::ScalarMarker<$DB> for $T {}
        )+
    };
}

/// Unified protocol for decoding an entire row into a specific Rust type.
/// Supports both single scalar extraction and multi-column tuple mapping.
pub trait DecodeColumn<'a, DB: Database>: Sized {
    fn decode_column(row: &DB::Row<'a>) -> Result<Self>;
}

// =========================================================
// Blanket Implementation for Scalar Types
// =========================================================
/// Automatically implements `DecodeColumn` for any type marked as a scalar.
impl<'a, DB, T> DecodeColumn<'a, DB> for T
where
    DB: Database,
    T: FromValue<DB> + ScalarMarker<DB>,
    // Enforces that the platform-specific Row implementation is capable
    // of extracting this specific column type.
    DB::Row<'a>: GetColumn<'a, T>,
{
    fn decode_column(row: &DB::Row<'a>) -> Result<Self> {
        // When decoding directly into a single scalar type,
        // the extraction always defaults to the first column (index 0).
        GetColumn::get_column(row, 0)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! bind_decode_column_tuple_inner {
    ($DB:ty, $($T:ident),+) => {
        impl<'a, $($T),+> $crate::types::DecodeColumn<'a, $DB> for ($($T,)+)
        where
            // Enforces that the concrete Row type supports extracting every
            // single element type defined within the tuple.
            $( <$DB as $crate::types::Database>::Row<'a>: $crate::types::GetColumn<'a, $T> ),+
        {
            fn decode_column(row: &<$DB as $crate::types::Database>::Row<'a>) -> $crate::Result<Self> {
                let mut idx = 0;
                Ok((
                    $(
                        {
                            // Sequentially advances the column index for each tuple element.
                            let val = $crate::types::GetColumn::<'a, $T>::get_column(row, idx)?;
                            idx += 1;
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
macro_rules! bind_decode_columns_tuple {
    ($DB:ty) => {
        $crate::bind_decode_column_tuple_inner!($DB, T1);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3, T4);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3, T4, T5);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7, T8);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $crate::bind_decode_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
        );
        $crate::bind_decode_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32
        );
    };
}
