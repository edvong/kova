#![allow(unreachable_code, dead_code, unused)]

use kova::Error;
use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

#[derive(Default, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
}

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
    pub const id: Column<i32, Engine> = 
        kova::private::create_column("order", "id", true);
    
    pub const name: Column<String, Engine> = 
        kova::private::create_column("order", "name", false);
    
    pub const is_active: Column<bool, Engine> = 
        kova::private::create_column("order", "is_active", false);
}
