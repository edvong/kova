use crate::dsl::func::ExtTableFunction;
use crate::dsl::{ExtAssignment, ExtColumn, ExtFunction, ExtOperator, ExtVirtualColumn, OnConflict};
use kova_core::types::Extension;

pub struct SqliteExtension;

impl Extension<'_> for SqliteExtension {
    type Column<T> = ExtColumn<T>;
    type VirtualColumn<T> = ExtVirtualColumn<T>;
    type Operator<'a> = ExtOperator<'a>;
    type Function<'a> = ExtFunction<'a>;
    type TableFunction<'t> = ExtTableFunction<'t>;
    type OrderOperand = ();
    type Assignment<'t> = ExtAssignment;
    type GroupBy = ();
    type Locking = ();
    type OnConflict<'a> = OnConflict<'a>;
}
