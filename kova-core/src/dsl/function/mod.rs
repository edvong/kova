mod aggregate;
mod builder;
mod scalar;

pub use aggregate::*;
pub use builder::FunctionBuilder;
pub use scalar::*;

use crate::dsl::component::WithComponent;
use crate::dsl::{
    Column, Component, Direction, OrderOperand, Ordering, Projection, ProjectionExpr,
    StandardOperand,
};
use crate::types::{ColumnMeta, Database, Extension};
use std::marker::PhantomData;

pub type ExtFunction<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Function<'a>;

pub struct Function<'a, DB: Database> {
    pub(crate) inner: Component<StandardFunction<'a, DB>, ExtFunction<'a, DB>>,
}

impl<'a, DB: Database> WithComponent for Function<'a, DB> {
    type Standard = StandardFunction<'a, DB>;
    type Extension = ExtFunction<'a, DB>;

    fn from_std(std: Self::Standard) -> Self {
        Self {
            inner: Component::standard(std),
        }
    }

    fn from_ext(ext: Self::Extension) -> Self {
        Self {
            inner: Component::extension(ext),
        }
    }
}

pub enum StandardFunction<'a, DB: Database> {
    CountAll,
    Count(ColumnMeta, Quantifier),
    Sum(ColumnMeta, Quantifier),
    Max(ColumnMeta, Quantifier),
    Min(ColumnMeta, Quantifier),
    Avg(ColumnMeta, Quantifier),

    Length(ColumnMeta),
    Lower(ColumnMeta),
    Upper(ColumnMeta),
    Abs(ColumnMeta),

    #[doc(hidden)]
    _Marker(PhantomData<&'a DB>),
}

impl<'a, DB: Database> Function<'a, DB> {
    pub fn alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
        Projection {
            expr: ProjectionExpr::Function(self),
            alias: Some(col.meta),
        }
    }

    pub fn asc(self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(self)),
            direction: Some(Direction::Asc),
            nulls: None,
        }
    }

    pub fn desc(self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(self)),
            direction: Some(Direction::Desc),
            nulls: None,
        }
    }
}
