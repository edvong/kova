use crate::dsl::InsertRows;
use crate::types::ColumnMeta;

pub struct ValuesClause<'a, T> {
    pub(crate) columns: Vec<ColumnMeta>,
    pub(crate) rows: InsertRows<'a, T>,
}

impl<'a, T> ValuesClause<'a, T> {
    pub(crate) fn new(columns: Vec<ColumnMeta>, rows: InsertRows<'a, T>) -> Self {
        Self { columns, rows }
    }
}
