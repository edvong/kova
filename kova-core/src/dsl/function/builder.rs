use std::marker::PhantomData;
use crate::dsl::function::{Function, Quantifier};
use crate::dsl::{
    Column, Direction, OrderOperand, Ordering, Projection, ProjectionExpr, StandardOperand,
    WithComponent,
};
use crate::types::Database;

// ==========================================
// Scalar/Basic Function Builder
// ==========================================
pub trait FunctionBuilder<'a, DB: Database>: Sized {
    fn build_func(self) -> Function<'a, DB>;

    #[inline(always)]
    fn build_alias<U, X>(self, col: Column<'a, U, DB>) -> Projection<'a, DB, X> {
        Projection::new(
            ProjectionExpr::Function(self.build_func()),
            Some(col.meta()),
        )
    }

    #[inline(always)]
    fn build_asc<X>(self) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(self.build_func())),
            direction: Some(Direction::Asc),
            nulls: None,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    fn build_desc<X>(self) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(self.build_func())),
            direction: Some(Direction::Desc),
            nulls: None,
            _marker: PhantomData,
        }
    }
}

// ==========================================
// Aggregate Function Builder
// ==========================================
pub(crate) trait AggregateBuilder<'a, DB: Database>: FunctionBuilder<'a, DB> {
    fn build_aggr(self, quantifier: Quantifier) -> Function<'a, DB>;

    #[inline(always)]
    fn build_distinct(self) -> Function<'a, DB> {
        self.build_aggr(Quantifier::Distinct)
    }
}
