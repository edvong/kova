use crate::dsl::func::ExtSrfFunction;
use crate::dsl::{ExtAssignment, ExtFunction, ExtOperator};
use kova_core::types::Extension;

pub struct SqliteExtension;

impl Extension<'_> for SqliteExtension {
    type Operator<'a> = ExtOperator<'a>;
    type Function<'a> = ExtFunction<'a>;
    type TableFunction<'t> = ExtSrfFunction<'t>;
    type OrderOperand = ();
    type Assignment<'t> = ExtAssignment;
    type GroupBy = ();
    type Locking = ();
}
