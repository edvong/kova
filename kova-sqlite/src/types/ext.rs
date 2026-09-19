use crate::dsl::{ExtAssignment, ExtColumn, ExtFunction, ExtOperator, OnConflict};
use kova_core::types::Extension;

pub struct SqliteExtension;

impl Extension<'_> for SqliteExtension {
    type Column<'a, T: 'a> = ExtColumn<T>;
    type Operator<'a> = ExtOperator<'a>;
    type Function<'a> = ExtFunction<'a>;
    type OrderOperand = ();
    type Assignment<'t> = ExtAssignment;
    type GroupBy = ();
    type Locking = ();
    type OnConflict<'a> = OnConflict<'a>;
}
