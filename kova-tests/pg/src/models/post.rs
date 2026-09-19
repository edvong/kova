use kova::dsl::{Column, ExtColumn, Identifier};
use kova::types::{ColumnMeta, IdentifierMeta, Database, Engine, FromRow, Model, Relation};

pub struct Post;

impl Relation for Post {
    const RELATION_NAME: &'static str = "post";
}

impl FromRow for Post {
    type DB = Engine;
    fn from_row(_: &<Self::DB as Database>::Row<'_>) -> kova::Result<Self> {
        unimplemented!()
    }
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
    pub const id: Column<'a, i32, Engine> = 
        ExtColumn::create(IdentifierMeta::new("pos", "id"));
       
    pub const title: Column<'a, String, Engine> =
        ExtColumn::create(IdentifierMeta::new("pos", "title"));

    pub const content: Column<'a, String, Engine> =
        ExtColumn::create(IdentifierMeta::new("pos", "content"));

    pub const author_id: Column<'a, i32, Engine> =
        ExtColumn::create(IdentifierMeta::new("pos", "author_id"));
}
