use crate::dsl::{Column, Operator};
use kova_core::types::Extension;

pub struct PgExtension;

impl Extension<'_> for PgExtension {
    type Column<T> = Column<T>;
    type Operator<'o> = Operator;
}