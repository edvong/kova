use crate::dsl::{Column, Operator, Pagination};
use kova_core::types::Extension;

pub struct SqliteExtension;

impl Extension<'_> for SqliteExtension {
    type Column<T> = Column<T>;
    type Operator<'o> = Operator;
    type Pagination = Pagination;
    type Projection = ();
    type Function<'t> = ();
}
