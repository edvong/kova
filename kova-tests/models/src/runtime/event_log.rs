use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

#[derive(Model)]
pub struct EventLog {
    pub id: Uuid,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Timestamp,
    pub website: Url,
    pub metadata: HashMap<String, String>,
    pub cost: Decimal,
    pub payload: AuditPayload,
    pub labels: TagList,
}

#[derive(Debug, PartialEq)]
pub struct Decimal(pub i64);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditPayload {
    pub actor: String,
    pub level: i32,
}

#[derive(Debug, PartialEq)]
pub struct TagList(pub Vec<String>);