use crate::dsl::{Assignment, Column, OnConflict, Operator, Pagination};
use kova_core::types::Extension;

pub struct SqliteExtension;

impl Extension<'_> for SqliteExtension {
    type Column<T> = Column<T>;
    type Operator<'o> = Operator;
    type Function<'t> = ();
    type OrderOperand = ();
    type Assignment<'t> = Assignment;
    type GroupBy = ();
    type Locking = ();
    type Pagination = Pagination;
    type OnConflict<'a> = OnConflict<'a>;
}
