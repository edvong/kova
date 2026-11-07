use crate::types::{Sqlite, Value};
use kova_kernel::types::{FromValue, IntoValue};
use kova_kernel::{Error, Result, impl_platform_codec, impl_platform_encode_only};
use std::borrow::Cow;
use std::convert::TryFrom;

#[diagnostic::on_unimplemented(
    message = "type `{Self}` cannot be encoded as a SQLite parameter under codec `{X}`",
    label = "not encodable under codec `{X}`",
    note = "if `{Self}` is a custom type, implement `Encode<'a, {X}> for {Self}` and configure `client.codec::<{X}>()`.\nfor overflow-prone 64-bit unsigned/pointer types (`u64`, `usize`, `isize`), enable the `narrow-cast` feature."
)]
pub trait Encode<'a, X = ()> {
    fn encode(self) -> Result<Value<'a>>;
}

#[diagnostic::on_unimplemented(
    message = "type `{Self}` cannot be decoded from a SQLite value under codec `{X}`",
    label = "not decodable under codec `{X}`",
    note = "if `{Self}` is a custom type, implement `Decode<'a, {X}> for {Self}` and `decode_column!({X}, {Self})`.\nfor narrowing extractions (`i8`, `u8`, `i16`, `u16`, `u32`, `u64`, `usize`, `isize`, `f32`), enable the `narrow-cast` feature."
)]
pub trait Decode<X = ()>: Sized {
    fn decode(value: Value<'_>) -> Result<Self>;
}

// Standard SQLite Types (Full Encode + Decode + FromColumn)
impl_platform_codec!(Sqlite, i32, i64, f64, bool, String, Vec<u8>);

// Lossless Numeric Upcasting to i64 / f64 (Encode Only by default)
impl_platform_encode_only!(Sqlite, i8, u8, i16, u16, u32, f32);

// ==========================================
// Native Integer (Aligned with SQLite INTEGER i64) & Standard i32
// ==========================================
impl<'a> IntoValue<'a, Sqlite> for i64 {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Int8(self))
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a i64 {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Int8(*self))
    }
}

impl FromValue<Sqlite> for i64 {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Int8(i) => Ok(i),
            _ => Err(Error::TypeMismatch("Expected Int8 for i64".into())),
        }
    }
}

impl<'a> IntoValue<'a, Sqlite> for i32 {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Int8(i64::from(self)))
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a i32 {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Int8(i64::from(*self)))
    }
}

impl FromValue<Sqlite> for i32 {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Int8(i) => i32::try_from(i)
                .map_err(|_| Error::ParseError("Integer out of bounds for i32".into())),
            _ => Err(Error::TypeMismatch("Expected Int8 for i32".into())),
        }
    }
}

// ==========================================
// Lossless Integer & Float Upcasting (Infallible From)
// ==========================================
macro_rules! impl_sqlite_upcast_int_encode {
    ($($ty:ty),*) => {$(
            impl<'a> IntoValue<'a, Sqlite> for $ty {
                #[inline(always)]
                fn into_value(self) -> Result<Value<'a>> {
                    Ok(Value::Int8(i64::from(self)))
                }
            }

            impl<'a> IntoValue<'a, Sqlite> for &'a $ty {
                #[inline(always)]
                fn into_value(self) -> Result<Value<'a>> {
                    Ok(Value::Int8(i64::from(*self)))
                }
            }
        )*
    };
}

impl_sqlite_upcast_int_encode!(i8, u8, i16, u16, u32);

impl<'a> IntoValue<'a, Sqlite> for f32 {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Float8(f64::from(self)))
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a f32 {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Float8(f64::from(*self)))
    }
}

// ==========================================
// Native Float64 (SQLite REAL)
// ==========================================
impl<'a> IntoValue<'a, Sqlite> for f64 {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Float8(self))
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a f64 {
    #[inline(always)]
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Float8(*self))
    }
}

impl FromValue<Sqlite> for f64 {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Float8(f) => Ok(f),
            _ => Err(Error::TypeMismatch("Expected Float8 for f64".into())),
        }
    }
}

// ==========================================
// Boolean Type
// ==========================================
impl<'a> IntoValue<'a, Sqlite> for bool {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Int8(if self { 1 } else { 0 }))
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a bool {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Int8(if *self { 1 } else { 0 }))
    }
}

impl FromValue<Sqlite> for bool {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Int8(i) => Ok(i != 0),
            _ => Err(Error::TypeMismatch("Expected Int8 (0 or 1) for bool".into())),
        }
    }
}

// ==========================================
// Text Type (Strings)
// ==========================================
impl<'a> IntoValue<'a, Sqlite> for String {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Owned(self)))
    }
}

impl FromValue<Sqlite> for String {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Text(cow) => Ok(cow.into_owned()),
            _ => Err(Error::TypeMismatch("Expected Text for String".into())),
        }
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a String {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self.as_str())))
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a str {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self)))
    }
}

// ==========================================
// Binary Type (Blob) & Fixed-Size Arrays
// ==========================================
impl<'a> IntoValue<'a, Sqlite> for Vec<u8> {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Owned(self)))
    }
}

impl FromValue<Sqlite> for Vec<u8> {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Blob(cow) => Ok(cow.into_owned()),
            _ => Err(Error::TypeMismatch("Expected Blob for Vec<u8>".into())),
        }
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a Vec<u8> {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self.as_slice())))
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a [u8] {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self)))
    }
}

impl<'a, const N: usize> IntoValue<'a, Sqlite> for [u8; N] {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Owned(self.to_vec())))
    }
}

impl<'a, const N: usize> IntoValue<'a, Sqlite> for &'a [u8; N] {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self.as_slice())))
    }
}

impl<const N: usize> FromValue<Sqlite> for [u8; N] {
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
