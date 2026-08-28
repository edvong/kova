use crate::dsl::{Column, Function, Locking, Operator, Pagination};

pub struct Extension;

impl kova_core::types::Extension<'_> for Extension {
    type Column<T> = Column<T>;
    type Operator<'o> = Operator;
    type Function<'t> = Function<'t>;
    type OrderOperand = ();
    type GroupBy = ();
    type Locking = Locking;
    type Pagination = Pagination;
}
