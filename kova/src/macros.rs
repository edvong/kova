#[macro_export]
macro_rules! bind_type {
    // bind_type!(AppCodec, HashMap<String, String>)
    ($codec:ty, $type:ty) => {
        impl<'a> kova::types::FromColumn<'a, kova::types::Engine, $codec> for $type
            where
                $type: kova::types::Decode<'a, kova::types::Engine, $codec> {
            #[inline(always)]
            fn extract_next(
                row: &<kova::types::Engine as kova::types::Database>::Row<'a>, idx: &mut usize,
            ) -> kova::Result<Self> {
                Self::extract_value(row, idx)
            }
        }

        impl<'a> kova::types::FromColumn<'a, kova::types::Engine, $codec> for Option<$type>
            where
                $type: kova::types::Decode<'a, kova::types::Engine, $codec> {
            #[inline(always)]
            fn extract_next(
                row: &<kova::types::Engine as kova::types::Database>::Row<'a>, idx: &mut usize,
            ) -> kova::Result<Self> {
                Self::extract_by(
                    row,
                    idx,
                    <$type as kova::types::DecodeExt<'a, kova::types::Engine, $codec>>::decode_opt,
                )
            }
        }

        impl<'a> kova::types::FromColumn<'a, kova::types::Engine, $codec> for Box<$type>
            where
                $type: kova::types::Decode<'a, kova::types::Engine, $codec> {
            #[inline(always)]
            fn extract_next(
                row: &<kova::types::Engine as kova::types::Database>::Row<'a>, idx: &mut usize,
            ) -> kova::Result<Self> {
                Self::extract_by(
                    row,
                    idx,
                    <$type as kova::types::DecodeExt<'a, kova::types::Engine, $codec>>::decode_box,
                )
            }
        }

        #[cfg(feature = "pg")]
        impl<'a> kova::dsl::func::IntoSeriesOp<'a, $codec> for $type
            where
                $type: kova::types::Encode<'a, kova::types::Engine, $codec, $type> {
            #[inline(always)]
            fn into_series_op(self) -> kova::dsl::Operand<'a, kova::types::Engine> {
                kova::dsl::Operand::from_result(kova::types::Encode::encode(self))
            }
        }
    };
}
