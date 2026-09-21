use kova::Error;
use kova::dsl::{Column, ExtColumn};
use kova::types::{ColumnMeta, Database, Engine, FromRow, IdentifierMeta, Model, Relation};

#[derive(Default, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub balance: f64,
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
impl<'a> User {
    pub const id: Column<'a, i32, Engine> = ExtColumn::create(IdentifierMeta::new("user", "id"));

    pub const name: Column<'a, String, Engine> =
        ExtColumn::create(IdentifierMeta::new("user", "name"));

    pub const balance: Column<'a, f64, Engine> =
        ExtColumn::create(IdentifierMeta::new("user", "balance"));

    pub const is_active: Column<'a, bool, Engine> =
        ExtColumn::create(IdentifierMeta::new("user", "is_active"));
}
