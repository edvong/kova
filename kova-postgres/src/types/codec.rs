use crate::types::{Postgres, Value};
use kova_kernel::types::{FromValue, IntoValue};
use kova_kernel::{impl_platform_codec, impl_platform_encode_only, impl_platform_prim_codec, Error, Result};
use std::borrow::Cow;

#[diagnostic::on_unimplemented(
    message = "type `{Self}` cannot be encoded as a PostgreSQL parameter under codec `{X}`",
    label = "not encodable under codec `{X}`",
    note = "if `{Self}` is a custom type, implement `Encode<'a, {X}> for {Self}` and configure `client.codec::<{X}>()`.\nfor overflow-prone 64-bit unsigned/pointer types (`u64`, `usize`, `isize`), enable the `narrow-cast` feature."
)]
pub trait Encode<'a, X = ()> {
    fn encode(self) -> Result<Value<'a>>;
}

#[diagnostic::on_unimplemented(
    message = "type `{Self}` cannot be decoded from a PostgreSQL value under codec `{X}`",
    label = "not decodable under codec `{X}`",
    note = "if `{Self}` is a custom type, implement `Decode<'a, {X}> for {Self}` and `decode_column!({X}, {Self})`.\nfor narrowing integer extractions (`i8`, `u8`, `u16`, `u32`, `u64`, `usize`, `isize`), enable the `narrow-cast` feature."
)]
pub trait Decode<X = ()>: Sized {
    fn decode(value: Value<'_>) -> Result<Self>;
}

// 1:1 Native Wire Types (Full Encode + Decode + FromColumn)
impl_platform_codec!(Postgres, i16, i32, i64, f32, f64, bool, String, Vec<u8>);

// Lossless Integer Upcasting (Encode Only by default)
impl_platform_encode_only!(Postgres, i8, u8, u16, u32);

// ==========================================
// Native Aligned Types (Postgres Native Numeric & Bool)
// ==========================================
macro_rules! impl_native_type {
    ($ty:ty,$variant:ident) => {
        impl<'a> IntoValue<'a, Postgres> for $ty {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(self))
            }
        }

        impl<'a> IntoValue<'a, Postgres> for &'a $ty {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(*self))
            }
        }

        impl FromValue<Postgres> for $ty {
            fn from_value(value: Value<'_>) -> Result<Self> {
                match value {
                    Value::$variant(v) => Ok(v),
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

impl_native_type!(i16, Int2);
impl_native_type!(i32, Int4);
impl_native_type!(i64, Int8);
impl_native_type!(f32, Float4);
impl_native_type!(f64, Float8);
impl_native_type!(bool, Bool);

// ==========================================
// Lossless Upcast Numeric Types (Infallible From)
// ==========================================
macro_rules! impl_upcast_int_encode {
    ($ty:ty, $target:ty,$variant:ident) => {
        impl<'a> IntoValue<'a, Postgres> for $ty {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(<$target>::from(self)))
            }
        }

        impl<'a> IntoValue<'a, Postgres> for &'a $ty {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(<$target>::from(*self)))
            }
        }
    };
}

impl_upcast_int_encode!(i8, i16, Int2);
impl_upcast_int_encode!(u8, i16, Int2);
impl_upcast_int_encode!(u16, i32, Int4);
impl_upcast_int_encode!(u32, i64, Int8);

// ==========================================
// Text Types (Zero-copy & Ownership)
// ==========================================
impl<'a> IntoValue<'a, Postgres> for String {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Owned(self)))
    }
}

impl FromValue<Postgres> for String {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Text(cow) => Ok(cow.into_owned()),
            _ => Err(Error::TypeMismatch("Expected Text for String".into())),
        }
    }
}

impl<'a> IntoValue<'a, Postgres> for &'a String {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self)))
    }
}

impl<'a> IntoValue<'a, Postgres> for &'a str {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self)))
    }
}

// ==========================================
// Binary Types (Zero-copy & Ownership)
// ==========================================
impl<'a> IntoValue<'a, Postgres> for Vec<u8> {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Owned(self)))
    }
}

impl FromValue<Postgres> for Vec<u8> {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Blob(cow) => Ok(cow.into_owned()),
            _ => Err(Error::TypeMismatch("Expected Blob for Vec<u8>".into())),
        }
    }
}

impl<'a> IntoValue<'a, Postgres> for &'a Vec<u8> {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self.as_slice())))
    }
}

impl<'a> IntoValue<'a, Postgres> for &'a [u8] {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self)))
    }
}

// ==========================================
// Fixed-Size Arrays (e.g., UUID, Hashes)
// ==========================================
impl<'a, const N: usize> IntoValue<'a, Postgres> for [u8; N] {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Owned(self.to_vec())))
    }
}

impl<'a, const N: usize> IntoValue<'a, Postgres> for &'a [u8; N] {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self.as_slice())))
    }
}

impl<const N: usize> FromValue<Postgres> for [u8; N] {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Blob(cow) => cow
                .as_ref()
                .try_into()
                .map_err(|_| Error::ParseError(format!("Expected Blob of length {}", N))),
            _ => Err(Error::TypeMismatch("Expected Blob for fixed-size array".into())),
        }
    }
}


// ==========================================
// 1D Array Types (Zero-copy & Ownership)
// ==========================================
macro_rules! impl_native_array_type {
    ($ty:ty, $variant:ident) => {
        impl<'a> IntoValue<'a, Postgres> for Vec<$ty> {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(Cow::Owned(self)))
            }
        }

        impl<'a> IntoValue<'a, Postgres> for &'a Vec<$ty> {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(Cow::Borrowed(self.as_slice())))
            }
        }

        impl<'a> IntoValue<'a, Postgres> for &'a [$ty] {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(Cow::Borrowed(self)))
            }
        }

        impl<'a, const N: usize> IntoValue<'a, Postgres> for [$ty; N] {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(Cow::Owned(self.to_vec())))
            }
        }

        impl<'a, const N: usize> IntoValue<'a, Postgres> for &'a [$ty; N] {
            #[inline(always)]
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(Cow::Borrowed(self.as_slice())))
            }
        }

        impl<'a, 'b: 'a, X> Encode<'a, X> for &'b [$ty] {
            #[inline(always)]
            fn encode(self) -> Result<Value<'a>> {
                IntoValue::into_value(self)
            }
        }

        impl<'a, 'b: 'a, 'c, X> Encode<'a, X> for &'c &'b [$ty] {
            #[inline(always)]
            fn encode(self) -> Result<Value<'a>> {
                IntoValue::into_value(*self)
            }
        }

        impl<'a, X, const N: usize> Encode<'a, X> for [$ty; N] {
            #[inline(always)]
            fn encode(self) -> Result<Value<'a>> {
                IntoValue::into_value(self)
            }
        }

        impl<'a, 'b: 'a, X, const N: usize> Encode<'a, X> for &'b [$ty; N] {
            #[inline(always)]
            fn encode(self) -> Result<Value<'a>> {
                IntoValue::into_value(self)
            }
        }
    };
}

macro_rules! impl_native_array_from_value {
    ($ty:ty, $variant:ident) => {
        impl FromValue<Postgres> for Vec<$ty> {
            fn from_value(value: Value<'_>) -> Result<Self> {
                match value {
                    Value::$variant(cow) => Ok(cow.into_owned()),
                    _ => Err(Error::TypeMismatch(format!(
                        "Expected {} for Vec<{}>",
                        stringify!($variant),
                        stringify!($ty)
                    ))),
                }
            }
        }
    };
}

impl_native_array_type!(bool, BoolArray);
impl_native_array_type!(i16, Int2Array);
impl_native_array_type!(i32, Int4Array);
impl_native_array_type!(i64, Int8Array);
impl_native_array_type!(f32, Float4Array);
impl_native_array_type!(f64, Float8Array);
impl_native_array_type!(String, TextArray);
impl_native_array_type!(Vec<u8>, BlobArray);

impl_native_array_from_value!(bool, BoolArray);
impl_native_array_from_value!(i16, Int2Array);
impl_native_array_from_value!(i32, Int4Array);
impl_native_array_from_value!(i64, Int8Array);
impl_native_array_from_value!(f32, Float4Array);
impl_native_array_from_value!(f64, Float8Array);
impl_native_array_from_value!(Vec<u8>, BlobArray);

impl FromValue<Postgres> for Vec<String> {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::TextArray(cow) => Ok(cow.into_owned()),
            Value::StrArray(cow) => Ok(cow.iter().map(|&s| s.to_owned()).collect()),
            _ => Err(Error::TypeMismatch("Expected TextArray for Vec<String>".into())),
        }
    }
}

impl_platform_prim_codec!(
    Postgres,
    Vec<bool>, Vec<i16>, Vec<i32>, Vec<i64>, Vec<f32>, Vec<f64>, Vec<String>, Vec<Vec<u8>>,
);

// Zero-copy borrowed `&str` array / slice encodings into `Value::StrArray`
impl<'a> IntoValue<'a, Postgres> for Vec<&'a str> {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::StrArray(Cow::Owned(self)))
    }
}

impl<'a, 'b: 'a> IntoValue<'a, Postgres> for &'a Vec<&'b str> {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::StrArray(Cow::Borrowed(self.as_slice())))
    }
}

impl<'a, 'b: 'a> IntoValue<'a, Postgres> for &'a [&'b str] {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::StrArray(Cow::Borrowed(self)))
    }
}

impl<'a, const N: usize> IntoValue<'a, Postgres> for [&'a str; N] {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::StrArray(Cow::Owned(self.to_vec())))
    }
}

impl<'a, 'b: 'a, const N: usize> IntoValue<'a, Postgres> for &'a [&'b str; N] {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::StrArray(Cow::Borrowed(self.as_slice())))
    }
}

impl<'a, 'b: 'a, X> Encode<'a, X> for Vec<&'b str> {
    #[inline(always)]
    fn encode(self) -> Result<Value<'a>> {
        IntoValue::into_value(self)
    }
}

impl<'a, 'b: 'a, 'c: 'a, X> Encode<'a, X> for &'c Vec<&'b str> {
    #[inline(always)]
    fn encode(self) -> Result<Value<'a>> {
        IntoValue::into_value(self)
    }
}

impl<'a, 'b: 'a, 'c: 'a, X> Encode<'a, X> for &'c [&'b str] {
    #[inline(always)]
    fn encode(self) -> Result<Value<'a>> {
        IntoValue::into_value(self)
    }
}

impl<'a, 'b: 'a, 'c: 'a, 'd, X> Encode<'a, X> for &'d &'c [&'b str] {
    #[inline(always)]
    fn encode(self) -> Result<Value<'a>> {
        IntoValue::into_value(*self)
    }
}

impl<'a, 'b: 'a, X, const N: usize> Encode<'a, X> for [&'b str; N] {
    #[inline(always)]
    fn encode(self) -> Result<Value<'a>> {
        IntoValue::into_value(self)
    }
}

impl<'a, 'b: 'a, 'c: 'a, X, const N: usize> Encode<'a, X> for &'c [&'b str; N] {
    #[inline(always)]
    fn encode(self) -> Result<Value<'a>> {
        IntoValue::into_value(self)
    }
}