use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Result;
use kova::dsl::{Column, ExtColumn};
use kova::types::{ColumnMeta, Database, Engine, FromRow, IdentifierMeta, Model, Relation};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

#[derive(Clone)]
pub struct EventLog {
    pub id: Uuid,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Timestamp,
    pub website: Url,
    pub metadata: HashMap<String, String>,
}

impl FromRow for EventLog {
    type DB = Engine;
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> Result<Self> {
        unimplemented!()
    }
}

impl Relation for EventLog {
    const RELATION_NAME: &'static str = "event_log";
}

impl Model for EventLog {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(&'a self) -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl<'a> EventLog {
    pub const id: Column<'a, Uuid, Engine> =
        ExtColumn::create(IdentifierMeta::new("event_log", "id"));

    pub const description: Column<'a, String, Engine> =
        ExtColumn::create(IdentifierMeta::new("event_log", "id"));

    pub const created_at: Column<'a, DateTime<Utc>, Engine> =
        ExtColumn::create(IdentifierMeta::new("event_log", "id"));

    pub const updated_at: Column<'a, Timestamp, Engine> =
        ExtColumn::create(IdentifierMeta::new("event_log", "id"));

    pub const website: Column<'a, Url, Engine> =
        ExtColumn::create(IdentifierMeta::new("event_log", "id"));

    pub const metadata: Column<'a, HashMap<String, String>, Engine> =
        ExtColumn::create(IdentifierMeta::new("event_log", "id"));
}
