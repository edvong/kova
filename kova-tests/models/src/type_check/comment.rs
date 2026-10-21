use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation};

pub struct Comment;

impl Relation for Comment {
    const NAME: &'static str = "comment";
}

impl<X> FromRow<Engine, X> for Comment {
    fn from_row(_: &<Engine as Database>::Row<'_>) -> kova::Result<Self> {
        unimplemented!()
    }
}

impl<X> Model<Engine, X> for Comment {
    const COLUMNS_META: &'static [ColumnMeta] = &[];
    fn to_values<'a>(
        &'a self,
    ) -> kova::Result<Vec<(&'static str, <Engine as Database>::Value<'a>)>> {
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