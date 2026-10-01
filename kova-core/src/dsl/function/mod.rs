mod window;
mod macros;
#[rustfmt::skip]
mod scalar;
#[rustfmt::skip]
mod aggregate;

pub use crate::dsl::function::window::{FrameBound, FrameType, Window};
pub use crate::dsl::function::aggregate::{
    avg::Avg, count::Count, count::Star, max::Max, min::Min, sum::Sum,
};
pub use crate::dsl::function::scalar::{
    abs::Abs, length::Length, lower::Lower, upper::Upper,    
};

use crate::dsl::component::WithComponent;
use crate::dsl::{Component, Operand};
use crate::types::{Database, Extension};

pub type ExtFunction<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Function<'a>;

pub struct Function<'a, DB: Database> {
    pub(crate) inner: Component<StandardFunction<'a, DB>, ExtFunction<'a, DB>>,
}

impl<'a, DB: Database> WithComponent for Function<'a, DB> {
    type Standard = StandardFunction<'a, DB>;
    type Extension = ExtFunction<'a, DB>;

    fn from_std(std: Self::Standard) -> Self {
        Self { inner: Component::standard(std) }
    }

    fn from_ext(ext: Self::Extension) -> Self {
        Self { inner: Component::extension(ext) }
    }
}

pub enum StandardFunction<'a, DB: Database> {
    CountAll,
    Count(Box<Operand<'a, DB>>, Quantifier),
    Sum(Box<Operand<'a, DB>>, Quantifier),
    Max(Box<Operand<'a, DB>>, Quantifier),
    Min(Box<Operand<'a, DB>>, Quantifier),
    Avg(Box<Operand<'a, DB>>, Quantifier),

    Length(Box<Operand<'a, DB>>),
    Lower(Box<Operand<'a, DB>>),
    Upper(Box<Operand<'a, DB>>),
    Abs(Box<Operand<'a, DB>>),

    Window(Box<Window<'a, DB, ()>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    All,
    Distinct,
}

pub trait IntoFunction<'a, DB: Database>: Sized {
    fn into_func(self) -> Function<'a, DB>;
}

pub(crate) trait IntoAggregate<'a, DB: Database>: IntoFunction<'a, DB> {
    fn into_aggregate_with(self, quantifier: Quantifier) -> Function<'a, DB>;

    #[inline(always)]
    fn into_aggregate_distinct(self) -> Function<'a, DB> {
        self.into_aggregate_with(Quantifier::Distinct)
    }
}

// impl<'a, DB: Database> Function<'a, DB> {
//     pub fn alias<U, X>(self, col: Column<'a, U, DB>) -> Projection<'a, DB, X> {
//         Projection::new(
//             ProjectionExpr::Function(self),
//             Some(col.identifier())
//         )
//     }
//
//     pub fn asc<X>(self) -> Ordering<'a, DB, X> {
//         Ordering {
//             operand: OrderOperand::from_std(StandardOperand::Function(self)),
//             direction: Some(Direction::Asc),
//             nulls: None,
//             _marker:PhantomData,
//         }
//     }
//
//     pub fn desc<X>(self) -> Ordering<'a, DB, X> {
//         Ordering {
//             operand: OrderOperand::from_std(StandardOperand::Function(self)),
//             direction: Some(Direction::Desc),
//             nulls: None,
//             _marker:PhantomData,
//         }
//     }
// }
