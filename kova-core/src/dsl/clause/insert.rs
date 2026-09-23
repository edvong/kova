use crate::dsl::InsertRows;
use crate::dsl::statement::SelectStatement;
use crate::types::{ColumnMeta, Database};

pub struct ValuesClause<'a, DB: Database, T> {
    pub(crate) columns: Vec<ColumnMeta>,
    pub(crate) source: InsertSource<'a, DB, T>,
}

impl<'a, DB: Database, T> ValuesClause<'a, DB, T> {
    pub(crate) fn new(columns: Vec<ColumnMeta>, source: InsertSource<'a, DB, T>) -> Self {
        Self { columns, source }
    }
}

pub enum InsertSource<'a, DB: Database, T> {
    Rows(InsertRows<'a, T>),
    Select(Box<SelectStatement<'a, DB>>),
}
