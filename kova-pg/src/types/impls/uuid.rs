use kova_core::{Error, Result};

impl Type for crate::impls::uuid::Uuid {
    fn to_db<'a>(&'a self) -> std::result::Result<Value<'a>> {
        // tucked directly into the serialization escape hatch, since Uuid itself implements ToSql
        Ok(Value::PgNative(self))
    }

    fn from_db(value: Value<'_>) -> crate::Result<Self> {
        match value {
            // Intercepting binary stream from the deserialization escape pod
            Value::PgRawBytes { ty, raw } => {
                <crate::impls::uuid::Uuid as tokio_postgres::types::FromSql>::from_sql(&ty, raw)
                    .map_err(|e| Error::ParseError(e.to_string()))
            }
            _ => Err(Error::TypeMismatch("Expected PgRawBytes for Uuid".into())),
        }
    }
}