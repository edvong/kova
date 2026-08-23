use crate::types::{Sqlite, Value};
use kova_core::types::{FromValue, IntoValue};
use kova_core::{Error, Result};
use std::borrow::Cow;
use std::convert::TryFrom;

// ==========================================
// Native Integer (Aligned with SQLite INTEGER)
// ==========================================
/// SQLite natively supports up to 64-bit signed integers.
/// These implementations map Rust's `i64` directly to SQLite's `Int8` value without overhead.
impl<'a> IntoValue<'a, Sqlite> for i64 {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Int8(self))
    }
}

impl<'a> IntoValue<'a, Sqlite> for &'a i64 {
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

// ==========================================
// Other Integer Types (Safe Cast to i64)
// ==========================================
/// Macro to generate `IntoValue` and `FromValue` implementations for various integer types.
/// All conversions are strictly checked during runtime to prevent overflow or truncation,
/// ensuring safe mapping to SQLite's native `i64` (Int8).
macro_rules! impl_sqlite_int {
    ($($ty:ty),*) => {
        $(
            // Owned version: Takes ownership and safely casts to i64
            impl<'a> IntoValue<'a, Sqlite> for $ty {
                fn into_value(self) -> Result<Value<'a>> {
                    i64::try_from(self)
                        .map(Value::Int8)
                        .map_err(|_| Error::ConversionError(format!("Integer overflow converting {} to i64", stringify!($ty))))
                }
            }

            // Reference version: Auto-dereferences to seamlessly handle field borrows (e.g., `&user.age`)
            impl<'a> IntoValue<'a, Sqlite> for &'a $ty {
                fn into_value(self) -> Result<Value<'a>> {
                    i64::try_from(*self)
                        .map(Value::Int8)
                        .map_err(|_| Error::ConversionError(format!("Integer overflow converting &{} to i64", stringify!($ty))))
                }
            }

            // Deserialization version: Strict bounds checking to prevent large value truncation
            impl FromValue<Sqlite> for $ty {
                fn from_value(value: Value<'_>) -> Result<Self> {
                    match value {
                        Value::Int8(i) => <$ty>::try_from(i)
                            .map_err(|_| Error::ParseError(format!("Integer out of bounds for {}", stringify!($ty)))),
                        _ => Err(Error::TypeMismatch(format!("Expected Int8 for {}", stringify!($ty)))),
                    }
                }
            }
        )*
    };
}

impl_sqlite_int!(i8, u8, i16, u16, i32, u32, u64, isize, usize);

// ==========================================
// Floating-Point Numbers (Float / Real)
// ==========================================
/// Maps Rust floating-point numbers to SQLite's `REAL` (Float8) type.
/// `f32` is implicitly cast to `f64` for database storage.
macro_rules! impl_sqlite_float {
    ($($ty:ty),*) => {
        $(
            impl<'a> IntoValue<'a, Sqlite> for $ty {
                fn into_value(self) -> Result<Value<'a>> {
                    Ok(Value::Float8(self as f64))
                }
            }

            impl<'a> IntoValue<'a, Sqlite> for &'a $ty {
                fn into_value(self) -> Result<Value<'a>> {
                    Ok(Value::Float8(*self as f64))
                }
            }

            impl FromValue<Sqlite> for $ty {
                fn from_value(value: Value<'_>) -> Result<Self> {
                    match value {
                        Value::Float8(f) => Ok(f as $ty),
                        _ => Err(Error::TypeMismatch(format!("Expected Float8 for {}", stringify!($ty)))),
                    }
                }
            }
        )*
    };
}

impl_sqlite_float!(f32, f64);

// ==========================================
// Boolean Type
// ==========================================
/// SQLite does not have a native boolean storage class; it uses 0 (false) and 1 (true) as integers.
/// These implementations automatically handle the translation between Rust's `bool` and SQLite's `Int8(0|1)`.
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
            _ => Err(Error::TypeMismatch(
                "Expected Int8 (0 or 1) for bool".into(),
            )),
        }
    }
}

// ==========================================
// 5. Text Type (Strings)
// ==========================================
/// Handles UTF-8 string conversions. Utilizes `Cow` (Clone-on-Write) to enable
/// efficient zero-copy operations when borrowing directly from the database row.

// String: Takes ownership and allocates
impl<'a> IntoValue<'a, Sqlite> for String {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Owned(self)))
    }
}

// &'a str: Zero-copy borrowing (highly efficient for queries)
impl<'a> IntoValue<'a, Sqlite> for &'a str {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self)))
    }
}

// &'a String: Convenience conversion for references
impl<'a> IntoValue<'a, Sqlite> for &'a String {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self.as_str())))
    }
}

// Extract String from Value: Allocates memory only if the underlying `Cow` is borrowed
impl FromValue<Sqlite> for String {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Text(cow) => Ok(cow.into_owned()),
            _ => Err(Error::TypeMismatch("Expected Text for String".into())),
        }
    }
}

// ==========================================
// Binary Type (Blob)
// ==========================================
/// Handles raw byte array conversions mapping to SQLite's `BLOB`.
/// Similar to the Text type, it leverages `Cow` for optimized memory management and zero-copy reads.

// Vec<u8>: Takes ownership and allocates
impl<'a> IntoValue<'a, Sqlite> for Vec<u8> {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Owned(self)))
    }
}

// &'a [u8]: Zero-copy borrowing
impl<'a> IntoValue<'a, Sqlite> for &'a [u8] {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self)))
    }
}

// &'a Vec<u8>: Convenience conversion for references
impl<'a> IntoValue<'a, Sqlite> for &'a Vec<u8> {
    fn into_value(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Borrowed(self.as_slice())))
    }
}

// Extract Vec<u8> from Value
impl FromValue<Sqlite> for Vec<u8> {
    fn from_value(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Blob(cow) => Ok(cow.into_owned()),
            _ => Err(Error::TypeMismatch("Expected Blob for Vec<u8>".into())),
        }
    }
}

/// Fixed-size array support (e.g., cryptographic hashes, UUIDs like `[u8; 16]`, `[u8; 32]`).
/// Ensures the blob length exactly matches the expected array size during deserialization.
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
            Value::Blob(cow) => {
                let bytes = cow.into_owned();
                bytes
                    .try_into()
                    .map_err(|_| Error::ParseError(format!("Expected Blob of length {}", N)))
            }
            _ => Err(Error::TypeMismatch(
                "Expected Blob for fixed-size array".into(),
            )),
        }
    }
}
