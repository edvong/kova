use kova::Error;
use kova::codec::Encode;
use kova::dsl::Column;
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation, Row};

#[derive(Default, Clone)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub discount: Option<f64>,
    pub is_active: bool,
    pub raw_data: Vec<u8>,
}

impl Relation for Order {
    const NAME: &'static str = "order";
}

impl<X> FromRow<Engine, X> for Order {
    const COLUMN_COUNT: usize = 0;

    fn from_row_at(row: &<Engine as Database>::Row<'_>, idx: &mut usize) -> Result<Self, Error> {
        Ok(Self {
            id: row.get_column::<_, X>(idx, "id")?,
            user_id: row.get_column::<_, X>(idx, "user_id")?,
            amount: row.get_column::<_, X>(idx, "amount")?,
            discount: row.get_column::<_, X>(idx, "discount")?,
            is_active: row.get_column::<_, X>(idx, "is_active")?,
            raw_data: row.get_column::<_, X>(idx, "raw_data")?,
        })
    }
}

impl<X> Model<Engine, X> for Order {
    const COLUMNS_META: &'static [ColumnMeta] = &[
        ColumnMeta::new("order", "id", true),
        ColumnMeta::new("order", "user_id", false),
        ColumnMeta::new("order", "amount", false),
        ColumnMeta::new("order", "discount", false),
        ColumnMeta::new("order", "is_active", false),
        ColumnMeta::new("order", "raw_data", false),
    ];

    fn write_values<'a>(&'a self, out: &mut Vec<<Engine as Database>::Value<'a>>) -> Result<(), Error> {
        out.push(Encode::<'a, X>::encode(&self.user_id)?);
        out.push(Encode::<'a, X>::encode(&self.amount)?);
        out.push(Encode::<'a, X>::encode(&self.discount)?);
        out.push(Encode::<'a, X>::encode(&self.is_active)?);
        out.push(Encode::<'a, X>::encode(&self.raw_data)?);
        Ok(())
    }
}

#[allow(non_upper_case_globals)]
impl Order {
    pub const id: Column<i32> = Column::new("order", "id", true);
    pub const user_id: Column<i32> = Column::new("order", "user_id", false);
    pub const amount: Column<f64> = Column::new("order", "amount", false);
    pub const discount: Column<Option<f64>> = Column::new("order", "discount", false);
    pub const is_active: Column<bool> = Column::new("order", "is_active", false);
    pub const raw_data: Column<Vec<u8>> = Column::new("order", "raw_data", false);
    pub const created_at: Column<i32> = Column::new("order", "created_at", true);
}