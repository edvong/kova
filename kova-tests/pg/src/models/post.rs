use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

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
impl Post {
    pub const id: Column<i32> = Column::new("post", "id", false);
    pub const title: Column<String> = Column::new("post", "title", false);
    pub const content: Column<String> = Column::new("post", "content", false);
    pub const author_id: Column<i32> = Column::new("post", "author_id", false);
    pub const tags: Column<Vec<String>> = Column::new("post", "tags", false);
}