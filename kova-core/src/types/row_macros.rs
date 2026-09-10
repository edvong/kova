#[macro_export]
macro_rules! impl_native_extract_column {
    ($db:ty, $($type:ty),+) => {
        $(

            impl<'a, X> $crate::types::Encode<'a, $db, X, $type> for $type {
                #[inline(always)]
                fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                    $crate::types::IntoValue::into_value(self)
                }
            }

            impl<'a, 'b: 'a, X> $crate::types::Encode<'a, $db, X, $type> for &'b $type {
                #[inline(always)]
                fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                    $crate::types::IntoValue::into_value(self)
                }
            }

            impl<'a, X> $crate::types::Decode<'a, $db, X> for $type {
                #[inline(always)]
                fn decode(value: <$db as $crate::types::Database>::Value<'a>) -> $crate::Result<Self> {
                    $crate::types::FromValue::from_value(value)
                }
            }



            impl<'a, X> $crate::types::FromColumn<'a, $db, X> for $type
            where
                $type: $crate::types::Decode<'a, $db, X>,
            {
                #[inline(always)]
                fn extract_next(row: &<$db as $crate::types::Database>::Row<'a>, idx: &mut usize) -> $crate::Result<Self> {
                    Self::extract_value(row, idx)
                }
            }

            impl<'a, X> $crate::types::FromColumn<'a, $db, X> for Option<$type>
            where
                $type: $crate::types::Decode<'a, $db, X>, 
            {
                #[inline(always)]
                fn extract_next(row: &<$db as $crate::types::Database>::Row<'a>, idx: &mut usize) -> $crate::Result<Self> {
                    Self::extract_by(row, idx, <$type as $crate::types::DecodeExt<'a, $db, X>>::decode_opt)
                }
            }

            impl<'a, X> $crate::types::FromColumn<'a, $db, X> for Box<$type>
            where
                $type: $crate::types::Decode<'a, $db, X>,
            {
                #[inline(always)]
                fn extract_next(row: &<$db as $crate::types::Database>::Row<'a>, idx: &mut usize) -> $crate::Result<Self> {
                    Self::extract_by(row, idx, <$type as $crate::types::DecodeExt<'a, $db, X>>::decode_box)
                }
            }
        )+
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! bind_extract_column_tuple_inner {
    ($DB:ty, $($T:ident),+) => {
        #[diagnostic::do_not_recommend]
        impl<'a, $($T),+, X> $crate::types::FromColumn<'a, $DB, X> for ($($T,)+)
        where
            $( $T: $crate::types::FromColumn<'a, $DB, X> ),+
        {
            fn extract_next(
                row: &<$DB as $crate::types::Database>::Row<'a>,
                idx: &mut usize
            ) -> $crate::Result<Self> {
                Ok((
                    $(
                        $T::extract_next(row, idx)?,
                    )+
                ))
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! bind_extract_columns_tuple {
    ($DB:ty) => {
        $crate::bind_extract_column_tuple_inner!($DB, T1);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3, T4);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3, T4, T5);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7, T8);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $crate::bind_extract_column_tuple_inner!($DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
        );
        $crate::bind_extract_column_tuple_inner!(
            $DB, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
            T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32
        );
    };
}
