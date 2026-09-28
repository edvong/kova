#![allow(unreachable_code, dead_code, unused)]

use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::types::{Decode, Encode, Engine, Value};
use kova::{Error, Result, bind_type};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

pub struct AppCodec;

impl<'a> Encode<'a, Engine, AppCodec> for Uuid {
    fn encode(self) -> Result<Value<'a>> {
        // Ownership: Forced to allocate memory (16 bytes) because self is about to be dropped.
        Ok(Value::Blob(Cow::Owned(self.into_bytes().to_vec())))
    }
}
// Explicitly specify `Target = Uuid`
impl<'a, 'b: 'a> Encode<'a, Engine, AppCodec, Uuid> for &'b Uuid {
    fn encode(self) -> Result<Value<'a>> {
        // Zero-Copy: Strip 16-byte array references inside the UUID directly,
        // 0 memory allocation mapped to SQLite
        Ok(Value::Blob(Cow::Borrowed(self.as_bytes())))
    }
}
impl<'a> Decode<'a, Engine, AppCodec> for Uuid {
    fn decode(value: Value<'a>) -> Result<Self> {
        match value {
            Value::Blob(b) => Uuid::from_slice(&b).map_err(|e| Error::ParseError(e.to_string())),
            // Compatibility fallback: in case there is old data in the database that is stored strings
            Value::Text(t) => Uuid::parse_str(&t).map_err(|e| Error::ParseError(e.to_string())),
            _ => Err(Error::TypeMismatch(
                "Expected Blob (or Text) for Uuid".into(),
            )),
        }
    }
}
bind_type!(AppCodec, Uuid);


impl<'a> Encode<'a, Engine, AppCodec> for Url {
    fn encode(self) -> Result<Value<'a>> {
        // After taking ownership, wrap the String inside it by extracting it directly,
        // avoiding the extra .to_string() copies
        Ok(Value::Text(Cow::Owned(String::from(self))))
    }
}
impl<'a, 'b: 'a> Encode<'a, Engine, AppCodec, Url> for &'b Url {
    fn encode(self) -> Result<Value<'a>> {
        // Zero-copy: directly borrow &str,
        // which is already assembled inside the Url, and allocate 0 memory
        Ok(Value::Text(Cow::Borrowed(self.as_str())))
    }
}
impl<'a> Decode<'a, Engine, AppCodec> for Url {
    fn decode(value: Value<'a>) -> Result<Self> {
        match value {
            Value::Text(t) => Url::parse(&t).map_err(|e| Error::ParseError(e.to_string())),
            _ => Err(Error::TypeMismatch("Expected Text for Url".into())),
        }
    }
}
bind_type!(AppCodec, Url);


impl<'a> Encode<'a, Engine, AppCodec> for DateTime<Utc> {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Int8(self.timestamp_millis()))
    }
}
impl<'a, 'b: 'a> Encode<'a, Engine, AppCodec, DateTime<Utc>> for &'b DateTime<Utc> {
    fn encode(self) -> Result<Value<'a>> {
        // The number is copied, there is no heap allocation problem, the value is taken directly
        Ok(Value::Int8(self.timestamp_millis()))
    }
}
impl<'a> Decode<'a, Engine, AppCodec> for DateTime<Utc> {
    fn decode(value: Value<'a>) -> Result<Self> {
        match value {
            Value::Int8(ts) => DateTime::from_timestamp_millis(ts)
                .ok_or_else(|| Error::ParseError("Invalid timestamp".into())),
            _ => Err(Error::TypeMismatch("Expected Int8 for DateTime".into())),
        }
    }
}
bind_type!(AppCodec, DateTime<Utc>);


impl<'a> Encode<'a, Engine, AppCodec> for Timestamp {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Text(Cow::Owned(self.to_string())))
    }
}
impl<'a, 'b: 'a> Encode<'a, Engine, AppCodec, Timestamp> for &'b Timestamp {
    fn encode(self) -> Result<Value<'a>> {
        // Since Jiff Timestamp's structure is not a contiguous string buffer,
        // formatting necessarily allocates
        Ok(Value::Text(Cow::Owned(self.to_string())))
    }
}
impl<'a> Decode<'a, Engine, AppCodec> for Timestamp {
    fn decode(value: Value<'a>) -> Result<Self> {
        match value {
            Value::Text(t) => Timestamp::from_str(&t).map_err(|e| Error::ParseError(e.to_string())),
            _ => Err(Error::TypeMismatch(
                "Expected Text for Jiff Timestamp".into(),
            )),
        }
    }
}
bind_type!(AppCodec, Timestamp);


impl<'a> Encode<'a, Engine, AppCodec> for HashMap<String, String> {
    fn encode(self) -> Result<Value<'a>> {
        let json_str =
            serde_json::to_string(&self).map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(json_str)))
    }
}
// 'b: 'a constraint and Target = HashMap
impl<'a, 'b: 'a> Encode<'a, Engine, AppCodec, HashMap<String, String>>
    for &'b HashMap<String, String> {
    fn encode(self) -> Result<Value<'a>> {
        let json_str =
            serde_json::to_string(self).map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(json_str)))
    }
}
impl<'a> Decode<'a, Engine, AppCodec> for HashMap<String, String> {
    fn decode(value: Value<'a>) -> Result<Self> {
        match value {
            Value::Text(t) => {
                serde_json::from_str(&t).map_err(|e| Error::ParseError(e.to_string()))
            }
            _ => Err(Error::TypeMismatch("Expected Text for JSON HashMap".into())),
        }
    }
}
bind_type!(AppCodec, HashMap<String, String>);

impl<'a> Encode<'a, Engine, AppCodec> for Vec<i32> {
    fn encode(self) -> Result<Value<'a>> {
        let json_str = serde_json::to_string(&self)
            .map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(json_str)))
    }
}

impl<'a, 'b: 'a> Encode<'a, Engine, AppCodec, Vec<i32>> for &'b Vec<i32> {
    fn encode(self) -> Result<Value<'a>> {
        let json_str = serde_json::to_string(self)
            .map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(Value::Text(Cow::Owned(json_str)))
    }
}
