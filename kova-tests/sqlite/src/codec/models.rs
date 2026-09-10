use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Result;
use kova::dsl::{Column, ExtColumn};
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model};
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

impl Model for EventLog {
    const TABLE_NAME: &'static str = "event_log";
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(&'a self) -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>> {
        unimplemented!()
    }
}

impl<'a> EventLog {
    #[allow(non_upper_case_globals)]
    pub const id: Column<'a, Uuid, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "event_log",
        column_name: "id",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });

    #[allow(non_upper_case_globals)]
    pub const description: Column<'a, String, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "event_log",
        column_name: "description",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });

    #[allow(non_upper_case_globals)]
    pub const created_at: Column<'a, DateTime<Utc>, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "event_log",
        column_name: "created_at",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });

    #[allow(non_upper_case_globals)]
    pub const updated_at: Column<'a, Timestamp, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "event_log",
        column_name: "updated_at",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });

    #[allow(non_upper_case_globals)]
    pub const website: Column<'a, Url, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "event_log",
        column_name: "website",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });

    #[allow(non_upper_case_globals)]
    pub const metadata: Column<'a, HashMap<String, String>, Engine> =
        ExtColumn::create(ColumnMeta {
            table_name: "event_log",
            column_name: "metadata",
            is_autoincrement: false,
            is_json: true,
            is_ignore: false,
        });
}
