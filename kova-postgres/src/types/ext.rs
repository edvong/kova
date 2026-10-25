use crate::dsl::func::{ExtFunction, ExtSrfFunction};
use crate::dsl::{Constraint, ExtAssignment, ExtOperator, Locking};

pub struct Extension;

impl kova_core::types::Extension for Extension {
    type Operator<'a> = ExtOperator<'a>;
    type Function<'a> = ExtFunction<'a>;
    type TableFunction<'a> = ExtSrfFunction<'a>;
    type Assignment<'a> = ExtAssignment;
    type ConflictTarget = Constraint;
    type Locking = Locking;
}
