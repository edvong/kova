use crate::types::{Decode, Encode, Postgres, Value};
use kova_core::types::{FromValue, IntoValue};
use kova_core::{Error, Result, impl_platform_decode_only, impl_platform_prim_codec};
use std::convert::TryFrom;

// Narrowing Decode for types whose lossless Encode is already in codec.rs
impl_platform_decode_only!(Postgres, i8, u8, u16, u32);

// Full Encode + Decode + FromColumn for pointer-sized and 64-bit unsigned types
impl_platform_prim_codec!(Postgres, u64, isize, usize);

macro_rules! impl_narrow_int_decode {
    ($ty:ty, $variant:ident) => {
        impl FromValue<Postgres> for $ty {
            fn from_value(value: Value<'_>) -> Result<Self> {
                match value {
                    Value::$variant(v) => <$ty>::try_from(v).map_err(|_| {
                        Error::ParseError(format!(
                            "Out of bounds converting {} to {}",
                            stringify!($variant),
                            stringify!($ty)
                        ))
                    }),
                    Value::Null => Err(Error::UnexpectedNull(format!("Unexpected NULL for {}", stringify!($ty)))),
                    _ => Err(Error::TypeMismatch(format!(
                        "Expected {} for {}",
                        stringify!($variant),
                        stringify!($ty)
                    ))),
                }
            }
        }
    };
}

impl_narrow_int_decode!(i8, Int2);
impl_narrow_int_decode!(u8, Int2);
impl_narrow_int_decode!(u16, Int4);
impl_narrow_int_decode!(u32, Int8);

macro_rules! impl_checked_int64 {
    ($ty:ty) => {
        impl<'a> IntoValue<'a, Postgres> for $ty {
            fn into_value(self) -> Result<Value<'a>> {
                i64::try_from(self).map(Value::Int8).map_err(|_| {
                    Error::ConversionError(format!(
                        "Overflow casting {} to i64",
                        stringify!($ty)
                    ))
                })
            }
        }

        impl<'a> IntoValue<'a, Postgres> for &'a $ty {
            fn into_value(self) -> Result<Value<'a>> {
                i64::try_from(*self).map(Value::Int8).map_err(|_| {
                    Error::ConversionError(format!(
                        "Overflow casting &{} to i64",
                        stringify!($ty)
                    ))
                })
            }
        }

        impl_narrow_int_decode!($ty, Int8);
    };
}

impl_checked_int64!(u64);
impl_checked_int64!(isize);
impl_checked_int64!(usize);
