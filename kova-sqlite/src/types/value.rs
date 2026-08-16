use kova_core::Error;
use rusqlite::ToSql;
use rusqlite::types::{Null, ToSqlOutput, ValueRef};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Null,
    Int8(i64),
    Float8(f64),
    // Cow supports both zero-allocation borrowed values (from &self)
    // and dynamically owned values such as String.
    Text(Cow<'a, str>),
    Blob(Cow<'a, [u8]>),
}

impl<'a> Value<'a> {
    /// Constructs a Value directly from a lifetime-bound ValueRef,
    /// avoiding the FromSql trait and preserving zero-copy access to the
    /// underlying SQLite data.
    pub(crate) fn from_value_ref(val_ref: ValueRef<'a>) -> kova_core::Result<Self, Error> {
        match val_ref {
            ValueRef::Null => Ok(Value::Null),
            ValueRef::Integer(i) => Ok(Value::Int8(i)),
            ValueRef::Real(f) => Ok(Value::Float8(f)),
            ValueRef::Text(t) => {
                let s = std::str::from_utf8(t)
                    .map_err(|e| Error::ParseError(e.to_string()))?;
                // Borrow the memory referenced by SQLite's
                // underlying C buffer without allocating a new String.
                Ok(Value::Text(Cow::Borrowed(s)))
            }
            ValueRef::Blob(b) => Ok(Value::Blob(Cow::Borrowed(b))),
        }
    }
}

// Implements the underlying driver's ToSql trait.
// The Cow contents are passed through directly, preserving the zero-copy
// representation without introducing an unnecessary intermediate allocation.
impl<'a> ToSql for Value<'a> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            Value::Null => Ok(ToSqlOutput::from(Null)),
            Value::Int8(i) => Ok(ToSqlOutput::from(*i)),
            Value::Float8(f) => Ok(ToSqlOutput::from(*f)),
            Value::Text(t) => Ok(ToSqlOutput::from(t.as_ref())), // Cow dereferences to &str.
            Value::Blob(b) => Ok(ToSqlOutput::from(b.as_ref())), // Cow dereferences to &[u8].
        }
    }
}