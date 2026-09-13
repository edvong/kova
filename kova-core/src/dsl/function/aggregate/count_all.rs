#![allow(clippy::wrong_self_convention)]

use crate::dsl::condition::BuildCondition;
use crate::dsl::func::Function;
use crate::dsl::function::StandardFunction;
use crate::dsl::function::builder::BuildFunction;
use crate::dsl::{
    BuildOrdering, BuildProjection, Column, Condition, Direction, IntoAlias, IntoArg, IntoOperand,
    Operand, OrderOperand, Ordering, Projection, ProjectionExpr, SingleArgMarker, StandardOperand,
    StandardOperator, WithComponent,
};
use crate::types::Database;
use std::marker::PhantomData;

pub struct CountAll;

impl<'a, DB: Database> BuildCondition<'a, DB> for CountAll {
    #[inline(always)]
    fn build_operand(self) -> Operand<'a, DB> {
        Operand::Function(self.build_func())
    }
}

impl<'a, DB: Database> BuildFunction<'a, DB> for CountAll {
    #[inline(always)]
    fn build_func(self) -> Function<'a, DB> {
        Function::from_std(StandardFunction::CountAll)
    }
}

impl<'a, DB: Database, X> BuildProjection<'a, DB, X> for CountAll {
    #[inline(always)]
    fn build_alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB, X> {
        Projection::new(
            ProjectionExpr::Function(self.build_func()),
            Some(col.into_identifier()),
        )
    }
}

impl<'a, DB: Database, X> IntoArg<Projection<'a, DB, X>> for CountAll {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, DB, X> {
        Projection::new(
            ProjectionExpr::Function(self.build_func()),
            None,
        )
    }
}

impl<'a, DB: Database, X> BuildOrdering<'a, DB, X> for CountAll {
    #[inline(always)]
    fn build_ordering(self, direction: Option<Direction>) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(self.build_func())),
            direction,
            nulls: None,
            _marker: PhantomData,
        }
    }
}

impl SingleArgMarker for CountAll {}

type Target = i64;

impl CountAll {
    pub fn eq<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Eq)
    }

    pub fn ne<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Ne)
    }

    pub fn gt<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Gt)
    }

    pub fn ge<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Ge)
    }

    pub fn lt<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Lt)
    }

    pub fn le<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Le)
    }

    pub fn is_null<'a, X, DB: Database>(self) -> Condition<'a, DB, X> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn is_not_null<'a, X, DB: Database>(self) -> Condition<'a, DB, X> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn between<'a, DB: Database, L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, DB, X>
    where
        L: IntoOperand<'a, Target, DB, K, X>,
        U: IntoOperand<'a, Target, DB, K, X>,
    {
        self.ternary_op(lower, upper, StandardOperator::Between)
    }

    pub fn not_between<'a, DB: Database, L, U, K, X>(
        self,
        lower: L,
        upper: U,
    ) -> Condition<'a, DB, X>
    where
        L: IntoOperand<'a, Target, DB, K, X>,
        U: IntoOperand<'a, Target, DB, K, X>,
    {
        self.ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    pub fn in_list<'a, DB: Database, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, DB, K, X>,
    {
        self.list_op(values, StandardOperator::In)
    }

    pub fn not_in<'a, DB: Database, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, DB, K, X>,
    {
        self.list_op(values, StandardOperator::NotIn)
    }

    pub fn alias<DB: Database, U, X>(self, col: Column<U, DB>) -> Projection<DB, X> {
        self.build_alias(col)
    }

    pub fn asc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Asc))
    }

    pub fn desc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Desc))
    }
}
