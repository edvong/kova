#![allow(clippy::wrong_self_convention)]

use crate::dsl::condition::ConditionBuilder;
use crate::dsl::func::Function;
use crate::dsl::function::StandardFunction;
use crate::dsl::function::builder::FunctionBuilder;
use crate::dsl::{Column, Condition, IntoArg, IntoOperand, Operand, Ordering, Projection, ProjectionExpr, StandardOperator, WithComponent};
use crate::types::Database;

pub struct CountAll;

impl<'a, DB: Database> ConditionBuilder<'a, DB> for CountAll {
    #[inline(always)]
    fn build_operand(self) -> Operand<'a, DB> {
        Operand::Function(self.build_func())
    }
}

impl<'a, DB: Database> FunctionBuilder<'a, DB> for CountAll {
    #[inline(always)]
    fn build_func(self) -> Function<'a, DB> {
        Function::from_std(StandardFunction::CountAll)
    }
}

type Target = i64;

impl CountAll {
    pub fn eq<'a, V, X, DB: Database>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB>,
    {
        self.binary_op(v, StandardOperator::Eq)
    }

    pub fn ne<'a, V, X, DB: Database>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB>,
    {
        self.binary_op(v, StandardOperator::Ne)
    }

    pub fn gt<'a, V, X, DB: Database>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB>,
    {
        self.binary_op(v, StandardOperator::Gt)
    }

    pub fn ge<'a, V, X, DB: Database>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB>,
    {
        self.binary_op(v, StandardOperator::Ge)
    }

    pub fn lt<'a, V, X, DB: Database>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB>,
    {
        self.binary_op(v, StandardOperator::Lt)
    }

    pub fn le<'a, V, X, DB: Database>(self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB>,
    {
        self.binary_op(v, StandardOperator::Le)
    }

    pub fn is_null<'a, X, DB: Database>(self) -> Condition<'a, DB, X> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn is_not_null<'a, X, DB: Database>(self) -> Condition<'a, DB, X> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn between<'a, L, U, X, DB: Database>(self, lower: L, upper: U) -> Condition<'a, DB, X>
    where
        L: IntoOperand<'a, Target, DB>,
        U: IntoOperand<'a, Target, DB>,
    {
        self.ternary_op(lower, upper, StandardOperator::Between)
    }

    pub fn not_between<'a, L, U, X, DB: Database>(self, lower: L, upper: U) -> Condition<'a, DB, X>
    where
        L: IntoOperand<'a, Target, DB>,
        U: IntoOperand<'a, Target, DB>,
    {
        self.ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    pub fn in_list<'a, I, V, X, DB: Database>(self, values: I) -> Condition<'a, DB, X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, DB>,
    {
        self.list_op(values, StandardOperator::In)
    }

    pub fn not_in<'a, I, V, X, DB: Database>(self, values: I) -> Condition<'a, DB, X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, DB>,
    {
        self.list_op(values, StandardOperator::NotIn)
    }


    pub fn alias<DB: Database, U, X>(self, col: Column<U, DB>) -> Projection<DB, X> {
        self.build_alias(col)
    }

    pub fn asc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_asc()
    }

    pub fn desc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_desc()
    }

}

impl<'a, DB: Database, X> IntoArg<Projection<'a, DB, X>> for CountAll {
    fn into_arg(self) -> Projection<'a, DB, X> {
        Projection::new(
            ProjectionExpr::Function(self.build_func()),
            None
        )
    }
}

