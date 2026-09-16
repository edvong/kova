use kova_core::types::Value as CoreValue;
use std::borrow::Cow;
use std::error::Error;
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[derive(Debug)]
pub enum Value<'a> {
    /// Represents a SQL NULL value.
    Null,

    // ==========================================
    // Native Aligned Types
    // ==========================================
    /// Strictly aligns with PostgreSQL's primitive numeric types for zero-overhead mapping.
    Bool(bool),
    Int2(i16),
    Int4(i32),
    Int8(i64),
    Float4(f32),
    Float8(f64),

    // ==========================================
    // Text & Binary Types (Zero-copy)
    // ==========================================
    /// Unified text representation mapping to types like `VARCHAR`, `TEXT`, `NAME`, or `XML`.
    /// Utilizes `Cow` to enable both owned strings and highly efficient zero-copy borrowing.
    Text(Cow<'a, str>),

    /// Unified binary representation mapping to `BYTEA` and other binary-compatible types.
    /// Utilizes `Cow` to enable both owned vectors and zero-copy byte slice borrows.
    Blob(Cow<'a, [u8]>),

    // ==========================================
    // Extension / Custom Types Escape Hatches
    // ==========================================
    /// Escape hatch for serialization (Rust -> Database) using a borrowed trait object.
    /// Used when delegating complex or driver-specific Rust types (e.g., Uuid, JsonB, Arrays)
    /// to the underlying tokio-postgres serialization mechanism.
    DynamicWrite(&'a (dyn ToSql + Sync)),

    /// Escape hatch for serialization with owned data.
    /// Functions identically to `DynamicWrite`, but takes ownership of the boxed trait object,
    /// which is crucial for dynamically constructed values requiring heap allocation.
    DynamicWriteOwned(Box<dyn ToSql + Sync + Send>),

    /// Escape hatch for deserialization (Database -> Rust).
    /// Captures the unparsed raw byte stream and PostgreSQL type metadata.
    ///
    /// The ORM's mapping layer intercepts this variant to decode the raw bytes
    /// into strongly-typed, potentially complex Rust structures (e.g., custom composite types).
    RawRead {
        ty: Type,
        raw: &'a [u8],
    },
}

impl<'a> FromSql<'a> for Value<'a> {
    /// Deserializes a value directly from the network buffer.
    /// The lifetime `'a` guarantees that zero-copy borrows (`&str`, `&[u8]`)
    /// safely reference the underlying socket stream without memory copying.
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

            // Fallback/Escape hatch: Any PostgreSQL type not explicitly mapped by the ORM
            // is preserved in its raw byte form along with its type OID metadata.
            _ => Ok(Value::RawRead { ty: ty.clone(), raw }),
        }
    }

    /// Acts as a universal type receiver, bypassing early type validation
    /// to let the `match` logic handle dynamic parsing or fallback to `RawRead`.
    fn accepts(_ty: &Type) -> bool {
        true
    }
}

impl<'a> ToSql for Value<'a> {
    fn to_sql(
        &self, ty: &Type, out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
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
            Value::Text(t) => t.as_ref().to_sql(ty, out),
            Value::Blob(b) => b.as_ref().to_sql(ty, out),

            // Dynamic dispatches: leverages tokio-postgres's robust native serializers.
            Value::DynamicWrite(native) => native.to_sql_checked(ty, out),
            Value::DynamicWriteOwned(native) => native.to_sql_checked(ty, out),

            // RawRead re-serialization: writes the unmodified bytes back to the buffer.
            Value::RawRead { raw, .. } => {
                out.extend_from_slice(raw);
                Ok(IsNull::No)
            }
        }
    }

    /// Universally accepts all types for flexible serialization.
    fn accepts(_ty: &Type) -> bool {
        true
    }

    fn to_sql_checked(
        &self, ty: &Type, out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
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
