use kova_kernel::types::Value as CoreValue;
use std::borrow::Cow;
use std::error::Error;
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[derive(Debug)]
pub enum Value<'a> {
    Null,

    // Native Aligned Types
    Bool(bool),
    Int2(i16),
    Int4(i32),
    Int8(i64),
    Float4(f32),
    Float8(f64),
    // mapping to types like `VARCHAR`, `TEXT`, `NAME`, or `XML`.
    Text(Cow<'a, str>),
    // mapping to `BYTEA` and other binary-compatible types.
    Blob(Cow<'a, [u8]>),

    // Native Aligned 1D Array Types
    BoolArray(Cow<'a, [bool]>),
    Int2Array(Cow<'a, [i16]>),
    Int4Array(Cow<'a, [i32]>),
    Int8Array(Cow<'a, [i64]>),
    Float4Array(Cow<'a, [f32]>),
    Float8Array(Cow<'a, [f64]>),
    TextArray(Cow<'a, [String]>),
    StrArray(Cow<'a, [&'a str]>),
    BlobArray(Cow<'a, [Vec<u8>]>),
}

impl<'a> Value<'a> {
    #[inline(always)]
    pub fn as_borrowed<'s>(&'s self) -> Value<'s>
        where
            'a: 's {
        match self {
            Value::Null => Value::Null,
            Value::Bool(b) => Value::Bool(*b),
            Value::Int2(i) => Value::Int2(*i),
            Value::Int4(i) => Value::Int4(*i),
            Value::Int8(i) => Value::Int8(*i),
            Value::Float4(f) => Value::Float4(*f),
            Value::Float8(f) => Value::Float8(*f),
            Value::Text(cow) => Value::Text(Cow::Borrowed(cow.as_ref())),
            Value::Blob(cow) => Value::Blob(Cow::Borrowed(cow.as_ref())),

            Value::BoolArray(cow) => Value::BoolArray(Cow::Borrowed(cow.as_ref())),
            Value::Int2Array(cow) => Value::Int2Array(Cow::Borrowed(cow.as_ref())),
            Value::Int4Array(cow) => Value::Int4Array(Cow::Borrowed(cow.as_ref())),
            Value::Int8Array(cow) => Value::Int8Array(Cow::Borrowed(cow.as_ref())),
            Value::Float4Array(cow) => Value::Float4Array(Cow::Borrowed(cow.as_ref())),
            Value::Float8Array(cow) => Value::Float8Array(Cow::Borrowed(cow.as_ref())),
            Value::TextArray(cow) => Value::TextArray(Cow::Borrowed(cow.as_ref())),
            Value::StrArray(cow) => Value::StrArray(Cow::Borrowed(cow.as_ref())),
            Value::BlobArray(cow) => Value::BlobArray(Cow::Borrowed(cow.as_ref())),
        }
    }

    #[inline]
    pub fn param_type(&self) -> Type {
        match self {
            Value::Null => Type::UNKNOWN,
            Value::Bool(_) => Type::BOOL,
            Value::Int2(_) => Type::INT2,
            Value::Int4(_) => Type::INT4,
            Value::Int8(_) => Type::INT8,
            Value::Float4(_) => Type::FLOAT4,
            Value::Float8(_) => Type::FLOAT8,
            Value::Text(_) => Type::UNKNOWN,
            Value::Blob(_) => Type::BYTEA,

            Value::BoolArray(_) => Type::BOOL_ARRAY,
            Value::Int2Array(_) => Type::INT2_ARRAY,
            Value::Int4Array(_) => Type::INT4_ARRAY,
            Value::Int8Array(_) => Type::INT8_ARRAY,
            Value::Float4Array(_) => Type::FLOAT4_ARRAY,
            Value::Float8Array(_) => Type::FLOAT8_ARRAY,
            Value::TextArray(_) | Value::StrArray(_) => Type::TEXT_ARRAY,
            Value::BlobArray(_) => Type::BYTEA_ARRAY,
        }
    }
}

impl<'a> FromSql<'a> for Value<'a> {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match *ty {
            // Numeric types: direct delegation to native tokio-postgres parsers.
            Type::INT2 => Ok(Value::Int2(i16::from_sql(ty, raw)?)),
            Type::INT4 => Ok(Value::Int4(i32::from_sql(ty, raw)?)),
            Type::INT8 => Ok(Value::Int8(i64::from_sql(ty, raw)?)),
            Type::FLOAT4 => Ok(Value::Float4(f32::from_sql(ty, raw)?)),
            Type::FLOAT8 => Ok(Value::Float8(f64::from_sql(ty, raw)?)),
            Type::BOOL => Ok(Value::Bool(bool::from_sql(ty, raw)?)),

            // Zero-copy path: borrows the raw slice directly from the TCP/network buffer,
            // bypassing heap allocations entirely for extreme read performance.
            Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::NAME => {
                let s = <&'a str>::from_sql(ty, raw)?;
                Ok(Value::Text(Cow::Borrowed(s)))
            }

            Type::JSON | Type::JSONB => {
                let mut b = raw;
                // If it is JSONB and there is data,
                // cut off the version control byte at the beginning (\x01)
                if *ty == Type::JSONB && !b.is_empty() {
                    if b[0] != 1 {
                        return Err("Unsupported JSONB encoding version".into());
                    }
                    b = &b[1..]; // Zero-Copy Slicing
                }

                let s = str::from_utf8(b)?;
                Ok(Value::Text(Cow::Borrowed(s)))
            }

            Type::BYTEA => {
                let b = <&'a [u8]>::from_sql(ty, raw)?;
                Ok(Value::Blob(Cow::Borrowed(b)))
            }

            Type::BOOL_ARRAY => Ok(Value::BoolArray(Cow::Owned(<Vec<bool>>::from_sql(ty, raw)?))),
            Type::INT2_ARRAY => Ok(Value::Int2Array(Cow::Owned(<Vec<i16>>::from_sql(ty, raw)?))),
            Type::INT4_ARRAY => Ok(Value::Int4Array(Cow::Owned(<Vec<i32>>::from_sql(ty, raw)?))),
            Type::INT8_ARRAY => Ok(Value::Int8Array(Cow::Owned(<Vec<i64>>::from_sql(ty, raw)?))),
            Type::FLOAT4_ARRAY => {
                Ok(Value::Float4Array(Cow::Owned(<Vec<f32>>::from_sql(ty, raw)?)))
            }
            Type::FLOAT8_ARRAY => {
                Ok(Value::Float8Array(Cow::Owned(<Vec<f64>>::from_sql(ty, raw)?)))
            }
            Type::TEXT_ARRAY | Type::VARCHAR_ARRAY | Type::BPCHAR_ARRAY | Type::NAME_ARRAY => {
                // Zero-copy element slices borrowed directly from the network buffer
                let slices = <Vec<&'a str>>::from_sql(ty, raw)?;
                Ok(Value::StrArray(Cow::Owned(slices)))
            }
            Type::BYTEA_ARRAY => {
                Ok(Value::BlobArray(Cow::Owned(<Vec<Vec<u8>>>::from_sql(ty, raw)?)))
            }
            _ => Err(format!("Unsupported PostgreSQL type: {ty}").into()),
        }
    }

    #[inline(always)]
    fn from_sql_null(_ty: &Type) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(Value::Null)
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }
}

impl<'a> ToSql for Value<'a> {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            Value::Null => Ok(IsNull::Yes),

            // Basic types: delegates directly to the native `ToSql` implementations.
            Value::Bool(b) => b.to_sql(ty, out),
            Value::Int2(i) => i.to_sql(ty, out),
            Value::Int4(i) => i.to_sql(ty, out),
            Value::Int8(i) => i.to_sql(ty, out),
            Value::Float4(f) => f.to_sql(ty, out),
            Value::Float8(f) => f.to_sql(ty, out),

            // Cow wrapped types: dereferences to `&str` or `&[u8]` before serialization.
            Value::Text(t) => {
                if *ty == Type::JSONB || *ty == Type::JSONPATH {
                    out.extend_from_slice(&[1]);
                }
                t.as_ref().to_sql(ty, out)
            }
            Value::Blob(b) => b.as_ref().to_sql(ty, out),

            Value::BoolArray(a) => a.as_ref().to_sql(ty, out),
            Value::Int2Array(a) => a.as_ref().to_sql(ty, out),
            Value::Int4Array(a) => a.as_ref().to_sql(ty, out),
            Value::Int8Array(a) => a.as_ref().to_sql(ty, out),
            Value::Float4Array(a) => a.as_ref().to_sql(ty, out),
            Value::Float8Array(a) => a.as_ref().to_sql(ty, out),
            Value::TextArray(a) => a.as_ref().to_sql(ty, out),
            Value::StrArray(a) => a.as_ref().to_sql(ty, out),
            Value::BlobArray(a) => a.as_ref().to_sql(ty, out),
        }
    }

    // Universally accepts all types for flexible serialization.
    fn accepts(_ty: &Type) -> bool {
        true
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut BytesMut)
        -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.to_sql(ty, out)
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
