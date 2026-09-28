use crate::dsl::{ExtAssignment, ExtColumn, ExtFunction, ExtOperator, Locking, OnConflict};
use crate::dsl::func::ExtTableFunction;

pub struct Extension;

impl kova_core::types::Extension<'_> for Extension {
    type Column<'a, T: 'a> = ExtColumn<'a, T>;
    type Operator<'a> = ExtOperator<'a>;
    type Function<'a> = ExtFunction<'a>;
    type TableFunction<'a> = ExtTableFunction<'a>;
    type OrderOperand = ();
    type Assignment<'a> = ExtAssignment;
    type GroupBy = ();
    type Locking = Locking;
    type OnConflict<'a> = OnConflict<'a>;
}
