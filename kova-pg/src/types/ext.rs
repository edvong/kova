use crate::dsl::func::{ExtFunction, ExtSrfFunction};
use crate::dsl::{ExtAssignment, ExtOperator, Locking};

pub struct Extension;

impl kova_core::types::Extension<'_> for Extension {
    type Operator<'a> = ExtOperator<'a>;
    type Function<'a> = ExtFunction<'a>;
    type TableFunction<'a> = ExtSrfFunction<'a>;
    type OrderOperand = ();
    type Assignment<'a> = ExtAssignment;
    type GroupBy = ();
    type Locking = Locking;
}
