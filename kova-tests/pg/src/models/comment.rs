use kova::dsl::{Column, ExtColumn};
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model};

pub struct Comment;

impl FromRow for Comment {
    type DB = Engine;
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> kova::Result<Self> {
        unimplemented!()
    }
}
impl Model for Comment {
    const TABLE_NAME: &'static str = "comment";
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(
        &'a self,
    ) -> kova::Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl<'a> Comment {
    pub const id: Column<'a, i32, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "comment",
        column_name: "id",
        is_autoincrement: true,
        is_json: false,
        is_ignore: false,
    });
    pub const post_id: Column<'a, i32, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "comment",
        column_name: "post_id",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });
    pub const user_id: Column<'a, i32, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "comment",
        column_name: "user_id",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });
    pub const content: Column<'a, String, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "comment",
        column_name: "content",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });
}
