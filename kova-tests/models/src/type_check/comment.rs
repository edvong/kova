use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};
use kova::Result;

pub struct Comment;

impl Relation for Comment {
    const NAME: &'static str = "comment";
}

impl<X> FromRow<Engine, X> for Comment {
    const COLUMN_COUNT: usize = 0;

    fn from_row_at(_: &<Engine as Database>::Row<'_>, _idx: &mut usize) -> Result<Self> {
        unimplemented!()
    }
}

impl<X> Model<Engine, X> for Comment {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn write_values<'a>( &'a self, _out: &mut Vec<<Engine as Database>::Value<'a>>) -> Result<()> {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
impl Comment {
    pub const id: Column<i32> = Column::new("comment", "id", false);
    pub const post_id: Column<i32> = Column::new("comment", "post_id", false);
    pub const user_id: Column<i32> = Column::new("comment", "user_id", false);
    pub const content: Column<String> = Column::new("comment", "content", false);
}