use kova::Error;
use kova::dsl::{Column, ExtColumn};
use kova::types::{ColumnMeta, Database, Engine, FromRow, IdentifierMeta, Model, Relation};

pub struct Order;

impl Relation for Order {
    const RELATION_NAME: &'static str = "order";
}

impl FromRow for Order {
    type DB = Engine;
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl Model for Order {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(
        &'a self,
    ) -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>, Error> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl<'a> Order {
    pub const id: Column<'a, i32, Engine> = ExtColumn::create(IdentifierMeta::new("order", "id"));

    pub const user_id: Column<'a, i32, Engine> =
        ExtColumn::create(IdentifierMeta::new("order", "user_id"));

    pub const amount: Column<'a, f64, Engine> =
        ExtColumn::create(IdentifierMeta::new("order", "amount"));

    pub const is_active: Column<'a, bool, Engine> =
        ExtColumn::create(IdentifierMeta::new("order", "is_active"));

    pub const raw_data: Column<'a, Vec<u8>, Engine> =
        ExtColumn::create(IdentifierMeta::new("order", "raw_data"));
}
