#[macro_export]
macro_rules! decode_column {
    // decode_column!(AppCodec, Uuid, Url, DateTime<Utc>, Timestamp);
    ($codec:ty, $($type:ty),+ $(,)?) => {
        $(
            impl<'a> $crate::types::FromColumn<'a, $crate::types::Engine, $codec> for $type
                where
                    $type: $crate::types::Decode<'a, $codec> {
                #[inline(always)]
                fn extract_next(
                    row: &<$crate::types::Engine as $crate::types::Database>::Row<'a>,
                    idx: &mut usize,
                ) -> $crate::Result<Self> {
                    Self::extract_value(row, idx)
                }
            }
        )+
    };
}

