#![allow(clippy::wrong_self_convention)]

use crate::dsl::function::{ExtFunction, IntoTableFunction};
use crate::dsl::jsonb_text::JsonbText;
use crate::dsl::operator::{ExtOperator, JsonOp};
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{Condition, IntoOperand, OpAst, Operand, Operator};
use std::marker::PhantomData;

pub struct JsonbChain<'a, T, X> {
    pub(crate) base: Operand<'a, Postgres>,
    pub(crate) ops: Vec<JsonOp<'a>>,
    pub(crate) _marker: PhantomData<(T, X)>,
}

// JsonbChain also an Operand
impl<'a, T, X> IntoOperand<'a, T, Postgres, OpAst, X> for JsonbChain<'a, T, X> {
    fn into_operand(self) -> Operand<'a, Postgres> {
        self.to_operand()
    }
}

impl<'a, U, X> IntoTableFunction<'a, X> for JsonbChain<'a, U, X> {
    #[inline(always)]
    fn into_table_function(self) -> Operand<'a, Postgres> {
        IntoOperand::into_operand(self)
    }
}

impl<'a, T, X> JsonbChain<'a, T, X> {
    #[inline(always)]
    fn to_operand(self) -> Operand<'a, Postgres> {
        // JsonChain is a function node that produces a value
        let func = ExtFunction::JsonChain { base: Box::new(self.base), ops: self.ops };
        Operand::from_function(Function::from(func))
    }

    // --- Concatenation method: append Vec directly and return Self ---
    pub fn json_path<V, K>(mut self, key: V) -> Self
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        self.ops.push(JsonOp::Path(key.into_operand()));
        self
    }

    pub fn json_path_array<I, V, K>(mut self, path: I) -> Self
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X> {
        self.ops.push(JsonOp::PathArray(ExtFunction::from_iter(path)));
        self
    }

    pub fn json_concat<V, K>(mut self, val: V) -> Self
        where
            V: IntoOperand<'a, T, Postgres, K, X> {
        self.ops.push(JsonOp::Concat(val.into_operand()));
        self
    }

    pub fn json_delete<V, K>(mut self, key: V) -> Self
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        self.ops.push(JsonOp::Delete(key.into_operand()));
        self
    }

    pub fn json_delete_path<I, V, K>(mut self, path: I) -> Self
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X> {
        self.ops.push(JsonOp::DeletePath(ExtFunction::from_iter(path)));
        self
    }

    // --- State transition: Switch to the Text extraction endpoint ---
    pub fn json_path_text<V, K>(mut self, key: V) -> JsonbText<'a, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        self.ops.push(JsonOp::PathText(key.into_operand()));
        JsonbText { base: self.base, ops: self.ops, _marker: PhantomData }
    }

    pub fn json_path_text_array<I, V, K>(mut self, path: I) -> JsonbText<'a, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Vec<String>, Postgres, K, X> {
        self.ops.push(JsonOp::PathTextArray(ExtFunction::from_iter(path)));
        JsonbText { base: self.base, ops: self.ops, _marker: PhantomData }
    }

    // --- JSONB-specific Boolean endpoint methods ---
    pub fn json_contains<V, K>(self, val: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, T, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonContains(
            // IntoOperand::<'a, T, Postgres, K, X>::into_operand(self),
            self.to_operand(),
            val.into_operand(),
        )))
    }

    pub fn json_contained_by<V, K>(self, val: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, T, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonContainedBy(
            self.to_operand(),
            val.into_operand(),
        )))
    }

    pub fn json_has_key<V, K>(self, key: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasKey(
            self.to_operand(),
            key.into_operand(),
        )))
    }

    pub fn json_has_any_keys<I, V, K>(self, keys: I) -> Condition<'a, Postgres, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasAnyKeys(
            self.to_operand(),
            ExtFunction::from_iter(keys),
        )))
    }

    pub fn json_has_all_keys<I, V, K>(self, keys: I) -> Condition<'a, Postgres, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasAllKeys(
            self.to_operand(),
            ExtFunction::from_iter(keys),
        )))
    }

    pub fn json_path_exists<V, K>(self, path: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonPathExists(
            self.to_operand(),
            path.into_operand(),
        )))
    }

    pub fn json_path_predicate<V, K>(self, path: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonPathPredicate(
            self.to_operand(),
            path.into_operand(),
        )))
    }
}
