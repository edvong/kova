use kova::Error;
use kova::dsl::{Column, Table};
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

#[derive(Default, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
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
impl<'a> User {
    pub const id: Column<i32, Engine> = 
        kova::private::create_column("user", "id", true);

    pub const name: Column<String, Engine> =
        kova::private::create_column("user", "name", false);

    pub const balance: Column<f64, Engine> =
        kova::private::create_column("user", "balance", false);

    pub const is_active: Column<bool, Engine> =
        kova::private::create_column("user", "is_active", false);
}
