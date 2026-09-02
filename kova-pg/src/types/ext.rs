use crate::dsl::{Assignment, Column, Function, Locking, OnConflict, Operator, Pagination};

pub struct Extension;

impl kova_core::types::Extension<'_> for Extension {
    type Column<T> = Column<T>;
    type Operator<'o> = Operator;
    type Function<'t> = Function<'t>;
    type OrderOperand = ();
    type Assignment<'t> = Assignment;
    type GroupBy = ();
    type Locking = Locking;
    type Pagination = Pagination;
    type OnConflict<'a> = OnConflict<'a>;
}
