use crate::types::{Postgres, Value};
use kova_core::types::{FromValue, IntoValue};
use kova_core::{Error, Result, impl_platform_codec};
use std::borrow::Cow;
use std::convert::TryFrom;

pub trait Encode<'a, X = ()> {
    fn encode(self) -> Result<Value<'a>>;
}

pub trait Decode<'a, X = ()>: Sized {
    fn decode(value: Value<'a>) -> Result<Self>;
}
#[rustfmt::skip]
impl_platform_codec!(
    Postgres,
    i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64, bool, String, Vec<u8>
);
// str, [u8]

// ==========================================
// Native Aligned Types (PG Native Numeric & Bool)
// ==========================================
/// Native Rust types that perfectly align with PostgreSQL's primitive types.
/// These implementations map directly to PG variants with zero overhead.
macro_rules! impl_native_type {
    ($ty:ty, $variant:ident) => {
        impl<'a> IntoValue<'a, Postgres> for $ty {
            fn into_value(self) -> Result<Value<'a>> {
                Ok(Value::$variant(self))
            }
        }

        impl<'a> IntoValue<'a, Postgres> for &'a $ty {
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
// Safe Cast Numeric Types
// ==========================================
/// Numeric types requiring safe casting.
/// All conversions undergo strict boundary checking at runtime to prevent
/// overflow or silent truncation when upcasting or downcasting to native PG types.
macro_rules! impl_cast_int {
    ($ty:ty, $target:ty, $variant:ident) => {
        impl<'a> IntoValue<'a, Postgres> for $ty {
            fn into_value(self) -> Result<Value<'a>> {
                <$target>::try_from(self).map(Value::$variant).map_err(|_| {
                    Error::ConversionError(format!(
                        "Overflow casting {} to {}",
                        stringify!($ty),
                        stringify!($target)
                    ))
                })
            }
        }

        impl<'a> IntoValue<'a, Postgres> for &'a $ty {
            fn into_value(self) -> Result<Value<'a>> {
                // Dereference *self to get value for safe conversion
                <$target>::try_from(*self).map(Value::$variant).map_err(|_| {
                    Error::ConversionError(format!(
                        "Overflow casting &{} to {}",
                        stringify!($ty),
                        stringify!($target)
                    ))
                })
            }
        }

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

// Upcasting (Zero data loss guaranteed)
impl_cast_int!(i8, i16, Int2);
impl_cast_int!(u8, i16, Int2);
impl_cast_int!(u16, i32, Int4);
impl_cast_int!(u32, i64, Int8);

// Platform pointer sizes with strict boundary checking against overflow
impl_cast_int!(isize, i64, Int8);
impl_cast_int!(usize, i64, Int8);
impl_cast_int!(u64, i64, Int8);

// ==========================================
// Text Types (Zero-copy & Ownership)
// ==========================================
/// Text types mapped to PostgreSQL's TEXT/VARCHAR.
/// Utilizes `Cow` (Clone-on-Write) to explicitly separate ownership from zero-copy borrowing.

// String: Takes ownership and allocates memory
impl<'a> IntoValue<'a, Postgres> for String {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Owned(self)))
    }
}

// Extract String from Value: Unwraps if Owned, allocates (clones) only if Borrowed
impl FromValue<Postgres> for String {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Text(cow) => Ok(cow.into_owned()),
            _ => Err(Error::TypeMismatch("Expected Text for String".into())),
        }
    }
}

// &'a String: Convenience conversion for references, wrapped as borrowed
impl<'a> IntoValue<'a, Postgres> for &'a String {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self)))
    }
}

// &'a str: Zero-copy borrowing. Wraps the slice in `Cow::Borrowed` without allocation.
impl<'a> IntoValue<'a, Postgres> for &'a str {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self)))
    }
}

/// Note: `&'a str` deliberately does not implement `FromValue`.
/// Since the lifetime of the database response is bound to the underlying network stream,
/// parsing directly into a borrowed `&str` is highly prone to dangling references.
/// Users should always extract text data safely using `String`.

// ==========================================
// Binary Types (Zero-copy & Ownership)
// ==========================================
/// Binary data mapped to PostgreSQL's BYTEA.
/// Leverages `Cow` for optimized memory management and zero-copy reads.

// Vec<u8>: Takes ownership of the heap allocation
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

// &'a Vec<u8>: Convenience conversion for references
impl<'a> IntoValue<'a, Postgres> for &'a Vec<u8> {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self.as_slice())))
    }
}

// &'a [u8]: Zero-copy borrowing. Avoids heap allocation by directly referencing the slice.
impl<'a> IntoValue<'a, Postgres> for &'a [u8] {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self)))
    }
}

// ==========================================
// Fixed-Size Arrays (e.g., UUID, Hashes)
// ==========================================
/// Support for fixed-size byte arrays, commonly used for raw UUID bytes or cryptographic hashes.
/// Ensures the blob length exactly matches the expected array size during deserialization.
impl<'a, const N: usize> IntoValue<'a, Postgres> for [u8; N] {
    fn into_value(self) -> Result<Value<'a>> {
        // Fixed-size array, consumed and converted to Vec for Cow.
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
            Value::Blob(cow) => {
                let bytes = cow.into_owned();
                bytes
                    .try_into()
                    .map_err(|_| Error::ParseError(format!("Expected Blob of length {}", N)))
            }
            _ => Err(Error::TypeMismatch("Expected Blob for fixed-size array".into())),
        }
    }
}
