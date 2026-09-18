use kova::dsl::{Column, ExtColumn};
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

pub struct Post;

impl FromRow for Post {
    type DB = Engine;
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> kova::Result<Self> {
        unimplemented!()
    }
}

impl Relation for Post { 
    const RELATION_NAME: &'static str = "post"; 
}

impl Model for Post {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(
        &'a self,
    ) -> kova::Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl<'a> Post {
    pub const id: Column<'a, i32, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "post",
        column_name: "id",
        is_autoincrement: true,
        is_json: false,
        is_ignore: false,
    });

    pub const title: Column<'a, String, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "post",
        column_name: "title",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });

    pub const content: Column<'a, String, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "post",
        column_name: "content",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });

    pub const author_id: Column<'a, i32, Engine> = ExtColumn::create(ColumnMeta {
        table_name: "post",
        column_name: "author_id",
        is_autoincrement: false,
        is_json: false,
        is_ignore: false,
    });
}
