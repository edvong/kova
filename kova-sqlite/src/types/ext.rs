use crate::dsl::{Column, Operator};
use kova_core::types::Extension;

pub struct SqliteExtension;

impl Extension<'_> for SqliteExtension {
    type Column<T> = Column<T>;
    type Operator<'o> = Operator;
}