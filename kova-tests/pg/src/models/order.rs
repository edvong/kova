use kova::Error;
use kova::dsl::{Column, ExtColumn};
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model};

pub struct Order;

impl FromRow for Order {
    type DB = Engine;
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl Model for Order {
    const TABLE_NAME: &'static str = "order";
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(
        &'a self,
    ) -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>, Error> {
        unimplemented!()
    }
}

impl<'a> Order {
    #[allow(non_upper_case_globals)]
    pub const id: Column<'a, i32, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "order",
        column_name: "id",
        is_autoincrement: true,
        is_json: false,
        is_ignore: false,
    });
    #[allow(non_upper_case_globals)]
    pub const user_id: Column<'a, i32, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "order",
        column_name: "user_id",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });
    #[allow(non_upper_case_globals)]
    pub const amount: Column<'a, f64, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "order",
        column_name: "amount",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });
    #[allow(non_upper_case_globals)]
    pub const is_active: Column<'a, bool, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "order",
        column_name: "is_active",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });
    #[allow(non_upper_case_globals)]
    pub const raw_data: Column<'a, Vec<u8>, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "order",
        column_name: "raw_data",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });
}
