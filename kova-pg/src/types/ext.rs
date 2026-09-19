use crate::dsl::{
    ExtAssignment, ExtColumn, ExtFunction, ExtOperator, Locking, OnConflict,
};

pub struct Extension;

impl kova_core::types::Extension<'_> for Extension {
    type Column<'c, T: 'c> = ExtColumn<'c, T>;
    type Operator<'o> = ExtOperator<'o>;
    type Function<'t> = ExtFunction<'t>;
    type OrderOperand = ();
    type Assignment<'t> = ExtAssignment;
    type GroupBy = ();
    type Locking = Locking;
    type OnConflict<'a> = OnConflict<'a>;
}
