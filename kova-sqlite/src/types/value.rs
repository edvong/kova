use kova_core::Error;
use kova_core::types::Value as CoreValue;
use rusqlite::ToSql;
use rusqlite::types::{Null, ToSqlOutput, ValueRef};
use std::borrow::Cow;

#[derive(Debug)]
pub enum Value<'a> {
    Null,

    // Native Aligned Types
    Int8(i64),
    Float8(f64),

    Text(Cow<'a, str>),
    Blob(Cow<'a, [u8]>),
}

impl<'a> Value<'a> {
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

    #[inline(always)]
    pub fn as_borrowed<'s>(&'s self) -> Value<'s>
        where
            'a: 's {
        match self {
            Value::Null => Value::Null,
            Value::Int8(i) => Value::Int8(*i),
            Value::Float8(f) => Value::Float8(*f),
            Value::Text(cow) => Value::Text(Cow::Borrowed(cow.as_ref())),
            Value::Blob(cow) => Value::Blob(Cow::Borrowed(cow.as_ref())),
        }
    }
}

impl<'a> ToSql for Value<'a> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            Value::Null => Ok(ToSqlOutput::from(Null)),
            Value::Int8(i) => Ok(ToSqlOutput::from(*i)),
            Value::Float8(f) => Ok(ToSqlOutput::from(*f)),
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
