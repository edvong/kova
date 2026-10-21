use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

pub struct Post;

impl Relation for Post {
    const NAME: &'static str = "post";
}

impl<X> FromRow<Engine, X> for Post {
    fn from_row(_: &<Engine as Database>::Row<'_>) -> kova::Result<Self> {
        unimplemented!()
    }
}

impl<X> Model<Engine, X> for Post {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(
        &'a self,
    ) -> kova::Result<Vec<(&'static str, <Engine as Database>::Value<'a>)>> {
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