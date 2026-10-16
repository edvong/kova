#[macro_export]
macro_rules! bind_type {
    // bind_type!(AppCodec, HashMap<String, String>)
    ($codec:ty, $type:ty) => {
        impl<'a> kova::types::FromColumn<'a, kova::types::Engine, $codec> for $type
            where
                $type: kova::types::Decode<'a, kova::types::Engine, $codec> {
            #[inline(always)]
            fn extract_next(row: &<kova::types::Engine as kova::types::Database>::Row<'a>, idx: &mut usize)
                -> kova::Result<Self> {
                Self::extract_value(row, idx)
            }
        }
    };
}
