use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Result;
use kova::codec::{Decode, Encode};
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation, Row};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;
use kova::dsl::Column;

#[derive(Clone)]
pub struct EventLog {
    pub id: Uuid,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Timestamp,
    pub website: Url,
    pub metadata: HashMap<String, String>,
}

impl Relation for EventLog {
    const NAME: &'static str = "event_log";
}

impl<X> FromRow<Engine, X> for EventLog
    where
        Uuid: Decode<X>,
        String: Decode<X>,
        DateTime<Utc>: Decode<X>,
        Timestamp: Decode<X>,
        Url: Decode<X>,
        HashMap<String, String>: Decode<X> {
    fn from_row_at(row: &<Engine as Database>::Row<'_>, idx: &mut usize) -> Result<Self> {
        Ok(Self {
            id: row.get_column::<_, X>(idx, "id")?,
            description: row.get_column::<_, X>(idx, "description")?,
            created_at: row.get_column::<_, X>(idx, "created_at")?,
            updated_at: row.get_column::<_, X>(idx, "updated_at")?,
            website: row.get_column::<_, X>(idx, "website")?,
            metadata: row.get_column::<_, X>(idx, "metadata")?,
        })
    }
}

impl<X> Model<Engine, X> for EventLog
    where
        Self: FromRow<Engine, X>,
        for<'a> &'a Uuid: Encode<'a, X>,
        for<'a> &'a String: Encode<'a, X>,
        for<'a> &'a DateTime<Utc>: Encode<'a, X>,
        for<'a> &'a Timestamp: Encode<'a, X>,
        for<'a> &'a Url: Encode<'a, X>,
        for<'a> &'a HashMap<String, String>: Encode<'a, X> {
    const COLUMNS_META: &'static [ColumnMeta] = &[
        ColumnMeta::new("event_log", "id", true),
        ColumnMeta::new("event_log", "description", false),
        ColumnMeta::new("event_log", "created_at", false),
        ColumnMeta::new("event_log", "updated_at", false),
        ColumnMeta::new("event_log", "website", false),
        ColumnMeta::new("event_log", "metadata", false),
    ];

    fn write_values<'a>(&'a self, out: &mut Vec<<Engine as Database>::Value<'a>>) -> Result<()> {
        out.push(Encode::<'a, X>::encode(&self.id)?);
        out.push(Encode::<'a, X>::encode(&self.description)?);
        out.push(Encode::<'a, X>::encode(&self.created_at)?);
        out.push(Encode::<'a, X>::encode(&self.updated_at)?);
        out.push(Encode::<'a, X>::encode(&self.website)?);
        out.push(Encode::<'a, X>::encode(&self.metadata)?);
        Ok(())
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

    // Columns for testing custom type Can*<AppCodec> and Option<CustomType> passthroughs
    pub const opt_website: Column<Option<Url>> = Column::new("event_log", "opt_website", false);
    pub const cost: Column<Decimal> = Column::new("event_log", "cost", false);
    pub const opt_cost: Column<Option<Decimal>> = Column::new("event_log", "opt_cost", false);
    pub const payload: Column<AuditPayload> = Column::new("event_log", "payload", false);
    pub const opt_payload: Column<Option<AuditPayload>> = Column::new("event_log", "opt_payload", false);
    pub const labels: Column<TagList> = Column::new("event_log", "labels", false);
    pub const opt_labels: Column<Option<TagList>> = Column::new("event_log", "opt_labels", false);
}

// Custom numeric domain type for testing `CanMath<AppCodec>`
#[derive(Clone, Copy, Default)]
pub struct Decimal(pub i64);

// Custom JSON domain struct for testing `CanPgJsonb<AppCodec>` / `CanSqliteJson<AppCodec>`
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuditPayload {
    pub actor: String,
    pub level: i32,
}

// Custom Array domain type for testing `CanPgArray<AppCodec>` (in Postgres)
#[derive(Clone, Default)]
pub struct TagList(pub Vec<String>);
