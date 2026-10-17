#![allow(unreachable_code, dead_code, unused)]

use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::dsl::{Compatible, Static};
use kova::types::{Decode, Encode, Engine, Value};
use kova::{Error, Result, decode_column};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

pub struct AppCodec;

// Memo: Custom cross-type compatibility rule under AppCodec.
// Allows passing raw `&str` or `String` directly into `Column<Url>` filters/assignments.
impl<'a> Compatible<Static<Url>, AppCodec> for Static<&'a str> {}
impl Compatible<Static<Url>, AppCodec> for Static<String> {}

impl<'a> Encode<'a, AppCodec> for Uuid {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Owned(self.into_bytes().to_vec())))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Uuid {
    fn encode(self) -> Result<Value<'a>> {
        // Zero-copy 16-byte slice borrow directly from Uuid
        Ok(Value::Blob(Cow::Borrowed(self.as_bytes())))
    }
}
impl<'a> Decode<'a, AppCodec> for Uuid {
    fn decode(value: Value<'a>) -> Result<Self> {
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
impl<'a> Decode<'a, AppCodec> for Url {
    fn decode(value: Value<'a>) -> Result<Self> {
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
impl<'a> Decode<'a, AppCodec> for DateTime<Utc> {
    fn decode(value: Value<'a>) -> Result<Self> {
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
impl<'a> Decode<'a, AppCodec> for Timestamp {
    fn decode(value: Value<'a>) -> Result<Self> {
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
impl<'a> Decode<'a, AppCodec> for HashMap<String, String> {
    fn decode(value: Value<'a>) -> Result<Self> {
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
        Ok(Value::DynamicWriteOwned(Box::new(self)))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Vec<i32> {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::DynamicWrite(self))
    }
}

impl<'a> Encode<'a, AppCodec> for Vec<String> {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::DynamicWriteOwned(Box::new(self)))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Vec<String> {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::DynamicWrite(self))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b [&'b str] {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::DynamicWriteOwned(Box::new(self)))
    }
}