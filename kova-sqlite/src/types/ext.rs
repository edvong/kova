use crate::dsl::func::ExtSrfFunction;
use crate::dsl::{ExtAssignment, ExtFunction, ExtOperator};
use kova_core::types::Extension;

pub struct SqliteExtension;

impl Extension for SqliteExtension {
    type Operator<'a> = ExtOperator<'a>;
    type Function<'a> = ExtFunction<'a>;
    type TableFunction<'a> = ExtSrfFunction<'a>;
    type Assignment<'a> = ExtAssignment;
    type ConflictTarget = ();
    type Locking = ();
}
