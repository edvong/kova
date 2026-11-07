#[macro_export]
macro_rules! decode_column {
    // decode_column!(AppCodec, Uuid, Url, DateTime<Utc>, Timestamp);
    ($codec:ty, $($type:ty),+ $(,)?) => {
        $(
            impl ::kova::codec::DecodeColumn<::kova::types::Engine, $codec> for $type
                where
                    $type: ::kova::codec::Decode<$codec> {
                #[inline(always)]
                fn extract_next(
                    row: &<::kova::types::Engine as ::kova::types::Database>::Row<'_>,
                    idx: &mut usize,
                ) -> ::kova::Result<Self> {
                    <Self as ::kova::codec::DecodeColumn<::kova::types::Engine, $codec>>::extract_value(row, idx)
                }
            }
        )+
    };
}