#[macro_export]
macro_rules! impl_native_extract_column {
    ($db:ty, $($type:ty),+) => {
        $(
            impl<'a, X> $crate::types::Encode<'a, $db, X> for $type {
                #[inline(always)]
                fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                    $crate::types::IntoValue::into_value(self)
                }
            }

            impl<'a, 'b: 'a, X> $crate::types::Encode<'a, $db, X> for &'b $type {
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
                    $type: $crate::types::Decode<'a, $db, X> {
                #[inline(always)]
                fn extract_next(row: &<$db as $crate::types::Database>::Row<'a>, idx: &mut usize) -> $crate::Result<Self> {
                    Self::extract_value(row, idx)
                }
            }
        )+
    };
}
