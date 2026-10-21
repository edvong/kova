#![allow(unreachable_code, dead_code, unused)]

use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::codec::*;
use kova::{Error, Result};
use models::type_check::event_log::{AuditPayload, Decimal};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

pub struct AppCodec;

// Memo: Custom cross-type compatibility rule under AppCodec.
// Allows passing `&str` or `String` directly into `Column<Url>` filters and assignments.
impl<'a> Compatible<Static<Url>, AppCodec> for Static<&'a str> {}
impl Compatible<Static<Url>, AppCodec> for Static<String> {}

impl<'a> Encode<'a, AppCodec> for Uuid {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Owned(self.into_bytes().to_vec())))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Uuid {
    fn encode(self) -> Result<Value<'a>> {
        // Zero-copy 16-byte array reference mapped directly to SQLite Blob
        Ok(Value::Blob(Cow::Borrowed(self.as_bytes())))
    }
}
impl Decode<AppCodec> for Uuid {
    fn decode(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Blob(b) => Uuid::from_slice(&b).map_err(|e| Error::ParseError(e.to_string())),
            Value::Text(t) => Uuid::parse_str(&t).map_err(|e| Error::ParseError(e.to_string())),
            _ => Err(Error::TypeMismatch("Expected Blob (or Text) for Uuid".into())),
        }
    }
}
decode_column!(AppCodec, Uuid);

impl<'a> Encode<'a, AppCodec> for Url {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Owned(String::from(self))))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Url {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Borrowed(self.as_str())))
    }
}
impl Decode<AppCodec> for Url {
    fn decode(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Text(t) => Url::parse(&t).map_err(|e| Error::ParseError(e.to_string())),
            _ => Err(Error::TypeMismatch("Expected Text for Url".into())),
        }
    }
}
decode_column!(AppCodec, Url);

impl<'a> Encode<'a, AppCodec> for DateTime<Utc> {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Int8(self.timestamp_millis()))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b DateTime<Utc> {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Int8(self.timestamp_millis()))
    }
}
impl Decode<AppCodec> for DateTime<Utc> {
    fn decode(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Int8(ts) => DateTime::from_timestamp_millis(ts)
                .ok_or_else(|| Error::ParseError("Invalid timestamp".into())),
            _ => Err(Error::TypeMismatch("Expected Int8 for DateTime".into())),
        }
    }
}
decode_column!(AppCodec, DateTime<Utc>);

impl<'a> Encode<'a, AppCodec> for Timestamp {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Owned(self.to_string())))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Timestamp {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Owned(self.to_string())))
    }
}
impl Decode<AppCodec> for Timestamp {
    fn decode(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Text(t) => Timestamp::from_str(&t).map_err(|e| Error::ParseError(e.to_string())),
            _ => Err(Error::TypeMismatch("Expected Text for Jiff Timestamp".into())),
        }
    }
}
decode_column!(AppCodec, Timestamp);

impl<'a> Encode<'a, AppCodec> for HashMap<String, String> {
    fn encode(self) -> Result<Value<'a>> {
        let json_str =
            serde_json::to_string(&self).map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(json_str)))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b HashMap<String, String> {
    fn encode(self) -> Result<Value<'a>> {
        let json_str =
            serde_json::to_string(self).map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(json_str)))
    }
}
impl Decode<AppCodec> for HashMap<String, String> {
    fn decode(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Text(t) => {
                serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string()))
            }
            _ => Err(Error::TypeMismatch("Expected Text for JSON HashMap".into())),
        }
    }
}
decode_column!(AppCodec, HashMap<String, String>);

impl<'a> Encode<'a, AppCodec> for Vec<i32> {
    fn encode(self) -> Result<Value<'a>> {
        let json_str =
            serde_json::to_string(&self).map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(json_str)))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Vec<i32> {
    fn encode(self) -> Result<Value<'a>> {
        let json_str =
            serde_json::to_string(self).map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(json_str)))
    }
}

// 1. Custom Textual Capability: Url -> CanString<AppCodec>
//    (Automatically unlocks CanSqliteString<AppCodec> & CanSqliteFts<AppCodec>)
impl CanString<AppCodec> for Url {}

// 2. Custom Numeric Capability: Decimal -> CanMath<AppCodec> + Compatible with i64
impl CanMath<AppCodec> for Decimal {}
impl Compatible<Static<Decimal>, AppCodec> for Static<i64> {}
impl Compatible<Static<Option<Decimal>>, AppCodec> for Static<i64> {}

impl<'a> Encode<'a, AppCodec> for Decimal {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Int8(self.0))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Decimal {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Int8(self.0))
    }
}
impl Decode<AppCodec> for Decimal {
    fn decode(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Int8(v) => Ok(Decimal(v)),
            _ => Err(Error::TypeMismatch("Expected Int8 for Decimal".into())),
        }
    }
}
decode_column!(AppCodec, Decimal);

// 3. Custom SQLite JSON Capability: AuditPayload -> CanSqliteJson<AppCodec>
impl CanSqliteJson<AppCodec> for AuditPayload {}

impl<'a> Encode<'a, AppCodec> for AuditPayload {
    fn encode(self) -> Result<Value<'a>> {
        let s = serde_json::to_string(&self).map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(s)))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b AuditPayload {
    fn encode(self) -> Result<Value<'a>> {
        let s = serde_json::to_string(self).map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(s)))
    }
}
impl Decode<AppCodec> for AuditPayload {
    fn decode(value: Value<'_>) -> Result<Self> {
        match value {
            Value::Text(t) => serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string())),
            _ => Err(Error::TypeMismatch("Expected Text for AuditPayload".into())),
        }
    }
}
decode_column!(AppCodec, AuditPayload);
