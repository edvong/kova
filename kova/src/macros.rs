#[macro_export]
macro_rules! bind_type {
    // bind_type!(AppCodec, HashMap<String, String>)
    ($codec:ty, $type:ty) => {
        impl<'a> kova::types::FromColumn<'a, kova::types::Engine, $codec> for $type
        where
            $type: kova::types::Decode<'a, kova::types::Engine, $codec>,
        {
            #[inline(always)]
            fn extract_next(
                row: &<kova::types::Engine as kova::types::Database>::Row<'a>,
                idx: &mut usize
            ) -> kova::Result<Self> {
                Self::extract_value(row, idx)
            }
        }

        impl<'a> kova::types::FromColumn<'a, kova::types::Engine, $codec> for Option<$type>
        where
            $type: kova::types::Decode<'a, kova::types::Engine, $codec>,
        {
            #[inline(always)]
            fn extract_next(
                row: &<kova::types::Engine as kova::types::Database>::Row<'a>,
                idx: &mut usize
            ) -> kova::Result<Self> {
                Self::extract_by(row, idx, <$type as kova::types::DecodeExt<'a, kova::types::Engine, $codec>>::decode_opt)
            }
        }

        impl<'a> kova::types::FromColumn<'a, kova::types::Engine, $codec> for Box<$type>
        where
            $type: kova::types::Decode<'a, kova::types::Engine, $codec>,
        {
            #[inline(always)]
            fn extract_next(
                row: &<kova::types::Engine as kova::types::Database>::Row<'a>,
                idx: &mut usize
            ) -> kova::Result<Self> {
                Self::extract_by(row, idx, <$type as kova::types::DecodeExt<'a, kova::types::Engine, $codec>>::decode_box)
            }
        }
    };
}