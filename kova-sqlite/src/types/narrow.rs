use crate::types::{Decode, Encode, Sqlite, Value};
use kova_core::types::{FromValue, IntoValue};
use kova_core::{Error, Result, impl_platform_decode_only, impl_platform_prim_codec};
use std::convert::TryFrom;

// 1. Narrowing Decode for types whose lossless Encode is already in codec.rs
impl_platform_decode_only!(Sqlite, i8, u8, i16, u16, u32, f32);

// 2. Full Encode + Decode + FromColumn for pointer-sized and 64-bit unsigned types
impl_platform_prim_codec!(Sqlite, u64, isize, usize);

macro_rules! impl_sqlite_narrow_int_decode {
    ($($ty:ty),*) => {
        $(
            impl FromValue<Sqlite> for $ty {
                fn from_value(value: Value<'_>) -> Result<Self> {
                    match value {
                        Value::Int8(i) => <$ty>::try_from(i).map_err(|_| {
                            Error::ParseError(format!(
                                "Integer out of bounds for {}",
                                stringify!($ty)
                            ))
                        }),
                        _ => Err(Error::TypeMismatch(format!(
                            "Expected Int8 for {}",
                            stringify!($ty)
                        ))),
                    }
                }
            }
        )*
    };
}

impl_sqlite_narrow_int_decode!(i8, u8, i16, u16, u32);

impl FromValue<Sqlite> for f32 {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Float8(f) => Ok(f as f32),
            _ => Err(Error::TypeMismatch("Expected Float8 for f32".into())),
        }
    }
}

macro_rules! impl_sqlite_checked_int64 {
    ($($ty:ty),*) => {
        $(
            impl<'a> IntoValue<'a, Sqlite> for $ty {
                fn into_value(self) -> Result<Value<'a>> {
                    i64::try_from(self).map(Value::Int8).map_err(|_| {
                        Error::ConversionError(format!(
                            "Integer overflow converting {} to i64",
                            stringify!($ty)
                        ))
                    })
                }
            }

            impl<'a> IntoValue<'a, Sqlite> for &'a $ty {
                fn into_value(self) -> Result<Value<'a>> {
                    i64::try_from(*self).map(Value::Int8).map_err(|_| {
                        Error::ConversionError(format!(
                            "Integer overflow converting &{} to i64",
                            stringify!($ty)
                        ))
                    })
                }
            }

            impl_sqlite_narrow_int_decode!($ty);
        )*
    };
}

impl_sqlite_checked_int64!(u64, isize, usize);
