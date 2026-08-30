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

impl CountAll {
    pub fn eq<'a, V, M, DB: Database>(self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Eq)
    }

    pub fn ne<'a, V, M, DB: Database>(self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Ne)
    }

    pub fn gt<'a, V, M, DB: Database>(self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Gt)
    }

    pub fn ge<'a, V, M, DB: Database>(self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Ge)
    }

    pub fn lt<'a, V, M, DB: Database>(self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Lt)
    }

    pub fn le<'a, V, M, DB: Database>(self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Le)
    }

    pub fn is_null<'a, DB: Database>(self) -> Condition<'a, DB> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn is_not_null<'a, DB: Database>(self) -> Condition<'a, DB> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn between<'a, L, M1, U, M2, DB: Database>(self, lower: L, upper: U) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, M1, DB>,
        U: IntoOperand<'a, M2, DB>,
    {
        self.ternary_op(lower, upper, StandardOperator::Between)
    }

    pub fn not_between<'a, L, M1, U, M2, DB: Database>(self, lower: L, upper: U) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, M1, DB>,
        U: IntoOperand<'a, M2, DB>,
    {
        self.ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    pub fn in_list<'a, I, V, M, DB: Database>(self, values: I) -> Condition<'a, DB>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, M, DB>,
    {
        self.list_op(values, StandardOperator::In)
    }

    pub fn not_in<'a, I, V, M, DB: Database>(self, values: I) -> Condition<'a, DB>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, M, DB>,
    {
        self.list_op(values, StandardOperator::NotIn)
    }


    pub fn alias<U, DB: Database>(self, col: Column<U, DB>) -> Projection<DB> {
        self.build_alias(col)
    }

    pub fn asc<'a, DB: Database>(self) -> Ordering<'a, DB> {
        self.build_asc()
    }

    pub fn desc<'a, DB: Database>(self) -> Ordering<'a, DB> {
        self.build_desc()
    }

}

impl<'a, DB: Database> IntoArg<Projection<'a, DB>> for CountAll {
    fn into_arg(self) -> Projection<'a, DB> {
        Projection {
            expr: ProjectionExpr::Function(self.build_func()),
            alias: None,
        }
    }
}

