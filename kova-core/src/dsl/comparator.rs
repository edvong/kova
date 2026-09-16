#![allow(clippy::wrong_self_convention)]

use crate::dsl::condition::Condition;
use crate::dsl::operand::IntoOperand;
use crate::dsl::operator::{Operator, StandardOperator};
use crate::dsl::{Operand, WithComponent};
use crate::types::Database;
use std::marker::PhantomData;

pub trait IntoComparator<'a, T, DB: Database> {
    fn into_comparator(self) -> Comparator<'a, T, DB>;
}

pub trait AsComparator<'a, T, DB: Database> {
    fn as_comparator(&self) -> Comparator<'a, T, DB>;
}

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
    fn binary_op<V, Target, X, K, F>(self, v: V, op_fn: F) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X>,
            F: FnOnce(Operand<'a, DB>, Operand<'a, DB>) -> StandardOperator<'a, DB>, {
        Condition::from(Operator::from_std(op_fn(self.operand, v.into_operand())))
    }

    #[inline(always)]
    fn unary_op<F, X>(self, op_fn: F) -> Condition<'a, DB, X>
        where
            F: FnOnce(Operand<'a, DB>) -> StandardOperator<'a, DB>, {
        Condition::from(Operator::from_std(op_fn(self.operand)))
    }

    #[inline(always)]
    fn ternary_op<L, U, Target, K, X, F>(
        self, lower: L, upper: U, op_fn: F,
    ) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, Target, DB, K, X>,
            U: IntoOperand<'a, Target, DB, K, X>,
            F: FnOnce(Operand<'a, DB>, Operand<'a, DB>, Operand<'a, DB>) -> StandardOperator<'a, DB>,
    {
        Condition::from(Operator::from_std(op_fn(
            self.operand,
            lower.into_operand(),
            upper.into_operand(),
        )))
    }

    #[inline(always)]
    fn list_op<I, V, Target, K, X, F>(self, values: I, op_fn: F) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, DB, K, X>,
            F: FnOnce(Operand<'a, DB>, Vec<Operand<'a, DB>>) -> StandardOperator<'a, DB>, {
        let vec = values.into_iter().map(|val| val.into_operand()).collect();
        Condition::from(Operator::from_std(op_fn(self.operand, vec)))
    }

    pub fn eq<V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X>, {
        self.binary_op(v, StandardOperator::Eq)
    }

    pub fn ne<V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X>, {
        self.binary_op(v, StandardOperator::Ne)
    }

    pub fn gt<V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X>, {
        self.binary_op(v, StandardOperator::Gt)
    }

    pub fn ge<V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X>, {
        self.binary_op(v, StandardOperator::Ge)
    }

    pub fn lt<V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X>, {
        self.binary_op(v, StandardOperator::Lt)
    }

    pub fn le<V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X>, {
        self.binary_op(v, StandardOperator::Le)
    }

    pub fn is_null<X>(self) -> Condition<'a, DB, X> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn is_not_null<X>(self) -> Condition<'a, DB, X> {
        self.unary_op(StandardOperator::IsNotNull)
    }

    pub fn between<L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, T, DB, K, X>,
            U: IntoOperand<'a, T, DB, K, X>, {
        self.ternary_op(lower, upper, StandardOperator::Between)
    }

    pub fn not_between<L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, T, DB, K, X>,
            U: IntoOperand<'a, T, DB, K, X>, {
        self.ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    pub fn in_list<I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T, DB, K, X>, {
        self.list_op(values, StandardOperator::In)
    }

    pub fn not_in_list<I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T, DB, K, X>, {
        self.list_op(values, StandardOperator::NotIn)
    }

    pub fn like<V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X>, {
        self.binary_op(v, StandardOperator::Like)
    }

    pub fn not_like<V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X>, {
        self.binary_op(v, StandardOperator::NotLike)
    }
}
