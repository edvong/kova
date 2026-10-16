use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Result;
use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};
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
    #[doc(hidden)]
    type DB = Engine;
    #[doc(hidden)]
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> Result<Self> {
        unimplemented!()
    }
}

impl Relation for EventLog {
    #[doc(hidden)]
    const RELATION_NAME: &'static str = "event_log";
}

impl Model for EventLog {
    #[doc(hidden)]
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    #[doc(hidden)]
    fn to_values<'a>(&'a self) -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl EventLog {
    pub const id: Column<Uuid> = Column::new("event_log", "id", true);
    pub const description: Column<String> = Column::new("event_log", "description", false);
    pub const created_at: Column<DateTime<Utc>> = Column::new("event_log", "created_at", false);
    pub const updated_at: Column<Timestamp> = Column::new("event_log", "updated_at", false);
    pub const website: Column<Url> = Column::new("event_log", "website", false);
    pub const metadata: Column<HashMap<String, String>> = Column::new("event_log", "metadata", false);
}