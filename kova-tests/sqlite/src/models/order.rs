use kova::Error;
use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

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
    pub const id: Column<i32, Engine> = 
        kova::private::create_column("order", "id", true);

    pub const user_id: Column<i32, Engine> = 
        kova::private::create_column("order", "user_id", false);

    pub const amount: Column<f64, Engine> = 
        kova::private::create_column("order", "amount", false);

    pub const is_active: Column<bool, Engine> = 
        kova::private::create_column("order", "is_active", false);

    pub const raw_data: Column<Vec<u8>, Engine> = 
        kova::private::create_column("order", "raw_data", false);
}
