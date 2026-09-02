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
    fn build_alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
        Projection {
            expr: ProjectionExpr::Function(self.build_func()),
            alias: Some(col.meta()),
        }
    }

    #[inline(always)]
    fn build_asc(self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(self.build_func())),
            direction: Some(Direction::Asc),
            nulls: None,
        }
    }

    #[inline(always)]
    fn build_desc(self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(self.build_func())),
            direction: Some(Direction::Desc),
            nulls: None,
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
