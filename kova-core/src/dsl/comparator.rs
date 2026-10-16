use crate::dsl::Operand;
use crate::dsl::condition::Condition;
use crate::dsl::operand::IntoOperand;
use crate::dsl::operator::{Operator, StandardOperator};
use crate::types::Database;
use std::marker::PhantomData;

pub struct Comparator<'a, T, DB: Database> {
    pub operand: Operand<'a, DB>,
    pub _marker: PhantomData<T>,
}

impl<'a, T, DB: Database> Comparator<'a, T, DB> {
    #[inline(always)]
    pub fn new(operand: Operand<'a, DB>) -> Self {
        Self { operand, _marker: PhantomData }
    }

    #[inline(always)]
    pub fn binary_op<V, K, X, F>(self, v: V, op_fn: F) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
        F: FnOnce(Operand<'a, DB>, Operand<'a, DB>) -> StandardOperator<'a, DB>, {
        Condition::from(Operator::from(op_fn(self.operand, v.into_operand())))
    }

    #[inline(always)]
    pub fn unary_op<X, F>(self, op_fn: F) -> Condition<'a, DB, X>
    where
        F: FnOnce(Operand<'a, DB>) -> StandardOperator<'a, DB>, {
        Condition::from(Operator::from(op_fn(self.operand)))
    }

    #[inline(always)]
    pub fn ternary_op<L, U, KL, KU, X, F>(
        self, lower: L, upper: U, op_fn: F,
    ) -> Condition<'a, DB, X>
    where
        L: IntoOperand<'a, T, DB, KL, X>,
        U: IntoOperand<'a, T, DB, KU, X>,
        F: FnOnce(Operand<'a, DB>, Operand<'a, DB>, Operand<'a, DB>) -> StandardOperator<'a, DB>,
    {
        Condition::from(Operator::from(op_fn(
            self.operand,
            lower.into_operand(),
            upper.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn list_op<I, V, K, X, F>(self, values: I, op_fn: F) -> Condition<'a, DB, X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, T, DB, K, X>,
        F: FnOnce(Operand<'a, DB>, Vec<Operand<'a, DB>>) -> StandardOperator<'a, DB>, {
        let vec = values.into_iter().map(|val| val.into_operand()).collect();
        Condition::from(Operator::from(op_fn(self.operand, vec)))
    }
}
