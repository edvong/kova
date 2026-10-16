#![allow(unreachable_code, dead_code, unused)]

use kova::Error;
use kova::dsl::{Column, Table};
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

#[derive(Default, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub bio: Option<String>,
    pub balance: f64,
    pub is_active: bool,
}

#[allow(non_upper_case_globals)]
pub const User: Table<User> = Table::new("user", "user");

impl Relation for User {
    const RELATION_NAME: &'static str = "user";
}

impl FromRow for User {
    type DB = Engine;
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl Model for User {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(
        &'a self,
    ) -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>, Error> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl User {
    pub const id: Column<i32> = Column::new("user", "id", true);
    pub const name: Column<String> = Column::new("user", "name", false);
    pub const bio: Column<Option<String>> = Column::new("user", "bio", false);
    pub const balance: Column<f64> = Column::new("user", "balance", false);
    pub const is_active: Column<bool> = Column::new("user", "is_active", false);
}
