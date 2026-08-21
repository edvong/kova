use std::borrow::Cow;
use std::error::Error;
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[derive(Debug)]
pub enum Value<'a> {
    Null,

    // Strictly maps to the corresponding PostgreSQL primitive numeric types.
    Bool(bool),
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
    // Extension / Custom Types Escape Hatches
    // ==========================================

    /// Escape hatch for serialization (Rust -> Database).
    /// Holds a dynamically dispatched `ToSql` trait object.
    ///
    /// Used when writing complex or database-specific Rust types
    /// (e.g., Uuid, JsonB, Arrays) to the database, allowing the
    /// underlying driver to handle the serialization natively.
    DynamicWrite(&'a (dyn ToSql + Sync)),

    /// Escape hatch for deserialization (Database -> Rust).
    /// Captures the unparsed raw byte stream and type metadata.
    ///
    /// Used when reading unknown or complex types from the database.
    /// The ORM's mapping layer (`FromValue`) intercepts this variant
    /// to decode the raw bytes into a strongly-typed Rust structure.
    RawRead {
        ty: Type,
        raw: &'a [u8]
    },
}

impl<'a> FromSql<'a> for Value<'a> {
    // 'a is the lifetime of the underlying network/socket buffer.
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
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
            _ => Ok(Value::RawRead {
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

impl<'a> ToSql for Value<'a> {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut)
        -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            Value::Null => Ok(IsNull::Yes),

            // Basic types: Directly reuse the ToSql implementation of the underlying types
            Value::Bool(b) => b.to_sql(ty, out),
            Value::Int2(i) => i.to_sql(ty, out),
            Value::Int4(i) => i.to_sql(ty, out),
            Value::Int8(i) => i.to_sql(ty, out),
            Value::Float4(f) => f.to_sql(ty, out),
            Value::Float8(f) => f.to_sql(ty, out),

            // Cow wrapped types: call after dereferencing to &str and &[u8].
            Value::Text(t) => t.as_ref().to_sql(ty, out),
            Value::Blob(b) => b.as_ref().to_sql(ty, out),

            Value::DynamicWrite(native) => native.to_sql_checked(ty, out),

            Value::RawRead { raw, .. } => {
                out.extend_from_slice(raw);
                Ok(IsNull::No)
            }
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut BytesMut)
        -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.to_sql(ty, out)
    }

}