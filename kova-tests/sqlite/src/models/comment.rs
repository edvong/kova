use kova::dsl::{Column, ExtColumn};
use kova::types::{ColumnMeta, Database, Engine, FromRow, IdentifierMeta, Model, Relation};

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
    pub const id: Column<'a, i32, Engine> =
        ExtColumn::create(IdentifierMeta::new("comment", "id"));
    pub const post_id: Column<'a, i32, Engine> =
        ExtColumn::create(IdentifierMeta::new("comment", "post_id"));
    pub const user_id: Column<'a, i32, Engine> =
        ExtColumn::create(IdentifierMeta::new("comment", "user_id"));
    pub const content: Column<'a, String, Engine> =
        ExtColumn::create(IdentifierMeta::new("comment", "content"));
}
