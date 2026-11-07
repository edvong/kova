use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

pub struct Post;

impl Relation for Post {
    const NAME: &'static str = "post";
}

impl<X> FromRow<Engine, X> for Post {
    const COLUMN_COUNT: usize = 0;

    fn from_row_at(_: &<Engine as Database>::Row<'_>, _idx: &mut usize) -> kova::Result<Self> {
        unimplemented!()
    }
}

impl<X> Model<Engine, X> for Post {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn write_values<'a>( &'a self, _out: &mut Vec<<Engine as Database>::Value<'a>>) -> kova::Result<()> {
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