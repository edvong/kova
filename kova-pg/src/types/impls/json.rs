use kova_core::{Error, Result};

impl Type for serde_json::Value {
    fn to_db<'a>(&'a self) -> crate::Result<Value<'a>> {
        Ok(Value::PgNative(self))
    }

    fn from_db(value: Value<'_>) -> crate::Result<Self> {
        match value {
            Value::PgRawBytes { ty, raw } => {
                <serde_json::Value as tokio_postgres::types::FromSql>::from_sql(&ty, raw)
                    .map_err(|e| Error::ParseError(e.to_string()))
            }
            _ => Err(Error::TypeMismatch("Expected PgRawBytes for JSON".into())),
        }
    }
}