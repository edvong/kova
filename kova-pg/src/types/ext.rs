use crate::dsl::{Column, Operator, Pagination};

pub struct Extension;

impl kova_core::types::Extension<'_> for Extension {
    type Column<T> = Column<T>;
    type Operator<'o> = Operator;
    type Pagination = Pagination;
    type Projection = ();
    type Function<'t> = ();
}
