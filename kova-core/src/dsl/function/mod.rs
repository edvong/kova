mod scalar;
mod aggregate;
mod builder;
mod macros;

pub use aggregate::*;
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






/*

macro_rules! impl_aggregate_function {
    ($StructName:ident, $Variant:ident) => {
        impl<'a, T, DB: Database> $StructName<Column<'a, T, DB>> {
            pub fn distinct(self) -> Function<'a, DB> {
                Function::from_std(StandardFunction::$Variant(
                    self.0.meta,
                    Quantifier::Distinct,
                ))
            }

            pub fn alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
                let func =
                    Function::from_std(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Projection {
                    expr: ProjectionExpr::Function(func),
                    alias: Some(col.meta),
                }
            }

            pub fn asc(self) -> Ordering<'a, DB> {
                let func =
                    Function::from_std(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Ordering {
                    operand: OrderOperand::from_std(StandardOperand::Function(func)),
                    direction: Some(Direction::Asc),
                    nulls: None,
                }
            }

            pub fn desc(self) -> Ordering<'a, DB> {
                let func =
                    Function::from_std(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Ordering {
                    operand: OrderOperand::from_std(StandardOperand::Function(func)),
                    direction: Some(Direction::Desc),
                    nulls: None,
                }
            }
        }

        impl<'a, T, DB: Database> IntoArg<Projection<'a, DB>> for $StructName<Column<'a, T, DB>> {
            fn into_arg(self) -> Projection<'a, DB> {
                let func =
                    Function::from_std(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Projection {
                    expr: ProjectionExpr::Function(func),
                    alias: None,
                }
            }
        }
    };
}

*/

// impl_aggregate_function!(Count, Count);
// impl_aggregate_function!(Sum, Sum);
// impl_aggregate_function!(Max, Max);
// impl_aggregate_function!(Min, Min);
// impl_aggregate_function!(Avg, Avg);

// ==== Scalar Functions ====

/*

macro_rules! impl_scalar_function {
    ($StructName:ident, $Variant:ident) => {
        impl<'a, T, DB: Database> $StructName<Column<'a, T, DB>> {
            pub fn alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
                let func = Function::from_std(StandardFunction::$Variant(self.0.meta));
                Projection {
                    expr: ProjectionExpr::Function(func),
                    alias: Some(col.meta),
                }
            }

            pub fn asc(self) -> Ordering<'a, DB> {
                let func = Function::from_std(StandardFunction::$Variant(self.0.meta));
                Ordering {
                    operand: OrderOperand::from_std(StandardOperand::Function(func)),
                    direction: Some(Direction::Asc),
                    nulls: None,
                }
            }

            pub fn desc(self) -> Ordering<'a, DB> {
                let func = Function::from_std(StandardFunction::$Variant(self.0.meta));
                Ordering {
                    operand: OrderOperand::from_std(StandardOperand::Function(func)),
                    direction: Some(Direction::Desc),
                    nulls: None,
                }
            }
        }

        impl<'a, T, DB: Database> IntoArg<Projection<'a, DB>> for $StructName<Column<'a, T, DB>> {
            fn into_arg(self) -> Projection<'a, DB> {
                let func = Function::from_std(StandardFunction::$Variant(self.0.meta));
                Projection {
                    expr: ProjectionExpr::Function(func),
                    alias: None,
                }
            }
        }
    };
}



 */
// impl_scalar_function!(Length, Length);
// impl_scalar_function!(Lower, Lower);
// impl_scalar_function!(Upper, Upper);
// impl_scalar_function!(Abs, Abs);
