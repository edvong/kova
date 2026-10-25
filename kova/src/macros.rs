#[macro_export]
macro_rules! decode_column {
    // decode_column!(AppCodec, Uuid, Url, DateTime<Utc>, Timestamp);
    ($codec:ty, $($type:ty),+ $(,)?) => {
        $(
            impl $crate::codec::DecodeColumn<$crate::types::Engine, $codec> for $type
                where
                    $type: $crate::codec::Decode<$codec> {
                #[inline(always)]
                fn extract_next(
                    row: &<$crate::types::Engine as $crate::types::Database>::Row<'_>,
                    idx: &mut usize,
                ) -> $crate::Result<Self> {
                    <Self as ::kova::codec::DecodeColumn<::kova::types::Engine, $codec>>::extract_value(row, idx)
                }
            }
        )+
    };
}