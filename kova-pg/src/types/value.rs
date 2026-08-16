use std::borrow::Cow;
use tokio_postgres::types::{FromSql, ToSql, Type};

#[derive(Debug)]
pub enum Value<'a> {
    Null,
    Bool(bool),

    // Strictly maps to the corresponding PostgreSQL primitive numeric types.
    Int2(i16),
    Int4(i32),
    Int8(i64),
    Float4(f32),
    Float8(f64),

    // Unified text representation for Varchar, Bpchar, Text, Name, Unknown, Xml, etc.
    Text(Cow<'a, str>),

    // Unified binary representation for Bytea and other binary-compatible types.
    Blob(Cow<'a, [u8]>),

    // ==========================================
    // Escape hatch for PostgreSQL-native values.
    // Handles Date, Time, Uuid, Json, Inet, Oid,
    // arrays, and any other types not explicitly handled above.
    // ==========================================
    PgNative(&'a (dyn ToSql + Sync)),
    PgRawBytes {
        ty: Type,
        raw: &'a [u8]
    },
}

impl<'a> FromSql<'a> for Value<'a> {
    // 'a is the lifetime of the underlying network/socket buffer.
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match *ty {
            // Numeric types: use the native PostgreSQL type parsers directly.
            Type::INT2 => Ok(Value::Int2(i16::from_sql(ty, raw)?)),
            Type::INT4 => Ok(Value::Int4(i32::from_sql(ty, raw)?)),
            Type::INT8 => Ok(Value::Int8(i64::from_sql(ty, raw)?)),
            Type::FLOAT4 => Ok(Value::Float4(f32::from_sql(ty, raw)?)),
            Type::FLOAT8 => Ok(Value::Float8(f64::from_sql(ty, raw)?)),
            Type::BOOL => Ok(Value::Bool(bool::from_sql(ty, raw)?)),

            // Zero-copy path: borrow the raw slice directly without any heap allocation.
            Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::NAME => {
                let s = <&'a str>::from_sql(ty, raw)?;
                Ok(Value::Text(Cow::Borrowed(s)))
            }
            Type::BYTEA => {
                let b = <&'a [u8]>::from_sql(ty, raw)?;
                Ok(Value::Blob(Cow::Borrowed(b)))
            }

            // Escape hatch: any PostgreSQL type not explicitly handled by Kova
            // is preserved as its original type metadata and raw byte representation.
            _ => Ok(Value::PgRawBytes {
                ty: ty.clone(),
                raw
            }),
        }
    }

    // Tell the underlying driver that this enum accepts values of any PostgreSQL type.
    fn accepts(_ty: &Type) -> bool {
        true
    }
}
