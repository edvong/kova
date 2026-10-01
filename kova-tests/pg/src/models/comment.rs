use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

pub struct Comment;

impl FromRow for Comment {
    type DB = Engine;
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> kova::Result<Self> {
        unimplemented!()
    }
}

impl Relation for Comment {
    const RELATION_NAME: &'static str = "comment";
}

impl Model for Comment {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(
        &'a self,
    ) -> kova::Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl<'a> Comment {
    pub const id: Column<i32, Engine> = 
        kova::private::create_column("comment", "id", false);
    pub const post_id: Column<i32, Engine> =
        kova::private::create_column("comment", "post_id", false);
    pub const user_id: Column<i32, Engine> =
        kova::private::create_column("comment", "user_id", false);
    pub const content: Column<String, Engine> =
        kova::private::create_column("comment", "content", false);
}
