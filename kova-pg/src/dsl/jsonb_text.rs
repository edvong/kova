#![allow(clippy::wrong_self_convention)]

use crate::dsl::operator::JsonOp;
use crate::dsl::{ExtFunction, ExtOperator};
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{Comparator, Condition, IntoComparator, IntoOperand, Operand, Operator, WithComponent};

pub struct JsonbText<'a> {
    pub(crate) base: Operand<'a, Postgres>,
    pub(crate) ops: Vec<JsonOp<'a>>,
}

impl<'a, T> IntoComparator<'a, T, Postgres> for JsonbText<'a> {
    #[inline(always)]
    fn into_comparator(self) -> Comparator<'a, T, Postgres> {
        Comparator::new(self.into_core_operand())
    }
}

impl<'a> JsonbText<'a> {
    #[inline(always)]
    fn into_core_operand(self) -> Operand<'a, Postgres> {
        let pg_func = ExtFunction::JsonChain {
            base: Box::new(self.base),
            ops: self.ops,
        };
        Operand::from_function(Function::from_ext(pg_func))
    }

    // --- Standard Operator Proxy ---
    pub fn eq<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().eq(v)
    }

    pub fn ne<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().ne(v)
    }

    pub fn gt<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().gt(v)
    }

    pub fn ge<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().ge(v)
    }

    pub fn lt<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().lt(v)
    }

    pub fn le<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().le(v)
    }

    pub fn is_null<X>(self) -> Condition<'a, Postgres, X> {
        IntoComparator::<'a, (), Postgres>::into_comparator(self).is_null()
    }

    pub fn is_not_null<X>(self) -> Condition<'a, Postgres, X> {
        IntoComparator::<'a, (), Postgres>::into_comparator(self).is_not_null()
    }

    pub fn like<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().like(v)
    }

    pub fn not_like<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().not_like(v)
    }

    pub fn in_list<I, V, K, X>(self, values: I) -> Condition<'a, Postgres, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().in_list(values)
    }

    pub fn not_in_list<I, V, K, X>(self, values: I) -> Condition<'a, Postgres, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        self.into_comparator().not_in_list(values)
    }

    // --- PG Proprietary Text Expansion Operators ---
    pub fn ilike<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        Condition::from(Operator::from_ext(ExtOperator::ILike(
            self.into_core_operand(),
            v.into_operand(),
        )))
    }

    pub fn not_ilike<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        Condition::from(Operator::from_ext(ExtOperator::NotILike(
            self.into_core_operand(),
            v.into_operand(),
        )))
    }

    pub fn matches<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        Condition::from(Operator::from_ext(ExtOperator::Matches(
            self.into_core_operand(),
            v.into_operand(),
        )))
    }

    pub fn not_matches<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        Condition::from(Operator::from_ext(ExtOperator::NotMatches(
            self.into_core_operand(),
            v.into_operand(),
        )))
    }

    pub fn imatches<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        Condition::from(Operator::from_ext(ExtOperator::IMatches(
            self.into_core_operand(),
            v.into_operand(),
        )))
    }

    pub fn not_imatches<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>,
    {
        Condition::from(Operator::from_ext(ExtOperator::NotIMatches(
            self.into_core_operand(),
            v.into_operand(),
        )))
    }
}
