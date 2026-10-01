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
impl<'a> Post {
    pub const id: Column<i32, Engine> = 
        kova::private::create_column("pos", "id", false);

    pub const title: Column<String, Engine> =
        kova::private::create_column("pos", "title", false);

    pub const content: Column<String, Engine> =
        kova::private::create_column("pos", "content", false);

    pub const author_id: Column<i32, Engine> =
        kova::private::create_column("pos", "author_id", false);
}
