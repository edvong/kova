use kova_core::Error;
use kova_core::types::Value as CoreValue;
use rusqlite::ToSql;
use rusqlite::types::{Null, ToSqlOutput, ValueRef};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    /// Represents a SQL NULL value.
    Null,

    // ==========================================
    // Native Aligned Types
    // ==========================================
    Int8(i64),
    Float8(f64),

    // ==========================================
    // Text & Binary Types (Zero-copy)
    // ==========================================
    /// Unified text representation.
    /// Utilizes `Cow` to support both dynamically owned strings and highly efficient zero-allocation borrows.
    Text(Cow<'a, str>),

    /// Unified binary representation.
    /// Utilizes `Cow` to support both owned vectors and zero-allocation byte slice borrows.
    Blob(Cow<'a, [u8]>),
}

impl<'a> Value<'a> {
    /// Constructs a `Value` directly from a lifetime-bound `ValueRef`.
    ///
    /// By bypassing the standard `FromSql` trait, this method preserves exact zero-copy
    /// access to the underlying SQLite internal data structures, maximizing read performance.
    pub(crate) fn from_value_ref(val_ref: ValueRef<'a>) -> kova_core::Result<Self, Error> {
        match val_ref {
            ValueRef::Null => Ok(Value::Null),
            ValueRef::Integer(i) => Ok(Value::Int8(i)),
            ValueRef::Real(f) => Ok(Value::Float8(f)),
            ValueRef::Text(t) => {
                let s = std::str::from_utf8(t).map_err(|e| Error::ParseError(e.to_string()))?;
                // Zero-copy path: Borrows the UTF-8 text directly from SQLite's
                // internal C buffer without incurring any heap allocation.
                Ok(Value::Text(Cow::Borrowed(s)))
            }
            ValueRef::Blob(b) => Ok(Value::Blob(Cow::Borrowed(b))),
        }
    }
}

/// Implements the underlying `rusqlite` driver's `ToSql` trait for seamless serialization.
impl<'a> ToSql for Value<'a> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            Value::Null => Ok(ToSqlOutput::from(Null)),
            Value::Int8(i) => Ok(ToSqlOutput::from(*i)),
            Value::Float8(f) => Ok(ToSqlOutput::from(*f)),

            // The Cow contents are explicitly dereferenced to `&str` and `&[u8]`.
            // This passes the zero-copy reference directly down to the driver
            // without introducing any unnecessary intermediate allocations.
            Value::Text(t) => Ok(ToSqlOutput::from(t.as_ref())),
            Value::Blob(b) => Ok(ToSqlOutput::from(b.as_ref())),
        }
    }
}

impl<'a> CoreValue for Value<'a> {
    fn none() -> Self {
        Value::Null
    }

    fn is_none(&self) -> bool {
        matches!(self, Value::Null)
    }
}
