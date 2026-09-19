use crate::dsl::{ExtAssignment, ExtColumn, OnConflict, Operator, Pagination};
use kova_core::types::Extension;

pub struct SqliteExtension;

impl Extension<'_> for SqliteExtension {
    type Column<'a, T: 'a> = ExtColumn<T>;
    type Operator<'o> = Operator;
    type Function<'t> = ();
    type OrderOperand = ();
    type Assignment<'t> = ExtAssignment;
    type GroupBy = ();
    type Locking = ();
    type Pagination = Pagination;
    type OnConflict<'a> = OnConflict<'a>;
}
