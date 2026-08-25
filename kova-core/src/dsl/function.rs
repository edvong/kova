use crate::dsl::arg::{IntoArg, IsSingleArg};
use crate::dsl::group_by::StandardGroupBy;
use crate::dsl::order::{Direction, OrderOperand, Ordering, StandardOperand};
use crate::dsl::projection::{Projection, StandardProjection};
use crate::dsl::{Column, Component, GroupBy};
use crate::types::{ColumnMeta, Database, Extension};
use std::marker::PhantomData;

pub type Function<'a, DB> = Component<
    StandardFunction<'a, DB>,
    <<DB as Database>::Extension<'a> as Extension<'a>>::Function<'a>,
>;

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

// Aggregate functions
pub struct CountAll;
pub struct Count<T>(pub T);
pub struct Sum<T>(pub T);
pub struct Max<T>(pub T);
pub struct Min<T>(pub T);
pub struct Avg<T>(pub T);

// Scalar functions
pub struct Length<T>(pub T);
pub struct Lower<T>(pub T);
pub struct Upper<T>(pub T);
pub struct Abs<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    All,
    Distinct,
}

// ==== Function to Projection ====

impl<'a, DB: Database> Function<'a, DB> {
    pub fn alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
        Projection::Standard(StandardProjection::Function(self, Some(col.meta)))
    }

    pub fn asc(self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::Standard(StandardOperand::Function(self)),
            direction: Direction::Asc,
            nulls: None,
        }
    }

    pub fn desc(self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::Standard(StandardOperand::Function(self)),
            direction: Direction::Desc,
            nulls: None,
        }
    }
}


// No Arguments, independent implementation
impl CountAll {
    pub fn alias<U, DB: Database>(self, col: Column<U, DB>) -> Projection<DB> {
        let func = Function::Standard(StandardFunction::CountAll);
        Projection::Standard(StandardProjection::Function(func, Some(col.meta)))
    }

    pub fn asc<'a, DB: Database>(self) -> Ordering<'a, DB> {
        let func = Function::Standard(StandardFunction::CountAll);
        Ordering {
            operand: OrderOperand::Standard(StandardOperand::Function(func)),
            direction: Direction::Asc,
            nulls: None,
        }
    }

    pub fn desc<'a, DB: Database>(self) -> Ordering<'a, DB> {
        let func = Function::Standard(StandardFunction::CountAll);
        Ordering {
            operand: OrderOperand::Standard(StandardOperand::Function(func)),
            direction: Direction::Desc,
            nulls: None,
        }
    }
}

impl<'a, DB: Database> IntoArg<Projection<'a, DB>> for CountAll {
    fn into_arg(self) -> Projection<'a, DB> {
        let func = Function::Standard(StandardFunction::CountAll);
        Projection::Standard(StandardProjection::Function(func, None))
    }
}


// ==== Aggregate Functions ====

macro_rules! impl_aggregate_function {
    ($StructName:ident, $Variant:ident) => {
        impl<'a, T, DB: Database> $StructName<Column<'a, T, DB>> {
            pub fn distinct(self) -> Function<'a, DB> {
                Component::Standard(StandardFunction::$Variant(
                    self.0.meta,
                    Quantifier::Distinct,
                ))
            }

            pub fn alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
                let func =
                    Component::Standard(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Component::Standard(StandardProjection::Function(func, Some(col.meta)))
            }

            pub fn asc(self) -> Ordering<'a, DB> {
                let func = Component::Standard(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Ordering {
                    operand: Component::Standard(StandardOperand::Function(func)),
                    direction: Direction::Asc,
                    nulls: None,
                }
            }

            pub fn desc(self) -> Ordering<'a, DB> {
                let func = Component::Standard(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Ordering {
                    operand: Component::Standard(StandardOperand::Function(func)),
                    direction: Direction::Desc,
                    nulls: None,
                }
            }
        }

        impl<'a, T, DB: Database> IntoArg<Projection<'a, DB>> for $StructName<Column<'a, T, DB>> {
            fn into_arg(self) -> Projection<'a, DB> {
                let func =
                    Function::Standard(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Projection::Standard(StandardProjection::Function(func, None))
            }
        }
    };
}

impl_aggregate_function!(Count, Count);
impl_aggregate_function!(Sum, Sum);
impl_aggregate_function!(Max, Max);
impl_aggregate_function!(Min, Min);
impl_aggregate_function!(Avg, Avg);

// ==== Scalar Functions ====

macro_rules! impl_scalar_function {
    ($StructName:ident, $Variant:ident) => {
        impl<'a, T, DB: Database> $StructName<Column<'a, T, DB>> {
            pub fn alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
                let func = Component::Standard(StandardFunction::$Variant(self.0.meta));
                Component::Standard(StandardProjection::Function(func, Some(col.meta)))
            }

            pub fn asc(self) -> Ordering<'a, DB> {
                let func = Component::Standard(StandardFunction::$Variant(self.0.meta));
                Ordering {
                    operand: Component::Standard(StandardOperand::Function(func)),
                    direction: Direction::Asc,
                    nulls: None,
                }
            }

            pub fn desc(self) -> Ordering<'a, DB> {
                let func = Component::Standard(StandardFunction::$Variant(self.0.meta));
                Ordering {
                    operand: Component::Standard(StandardOperand::Function(func)),
                    direction: Direction::Desc,
                    nulls: None,
                }
            }
        }

        impl<'a, T, DB: Database> IntoArg<Projection<'a, DB>> for $StructName<Column<'a, T, DB>> {
            fn into_arg(self) -> Projection<'a, DB> {
                let func = Function::Standard(StandardFunction::$Variant(self.0.meta));
                Projection::Standard(StandardProjection::Function(func, None))
            }
        }
    };
}

impl_scalar_function!(Length, Length);
impl_scalar_function!(Lower, Lower);
impl_scalar_function!(Upper, Upper);
impl_scalar_function!(Abs, Abs);
