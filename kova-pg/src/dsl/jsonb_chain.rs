#![allow(clippy::wrong_self_convention)]

use crate::dsl::function::ExtFunction;
use crate::dsl::jsonb_text::JsonbText;
use crate::dsl::operator::{ExtOperator, JsonOp};
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{Condition, IntoOperand, IsAst, Operand, Operator, WithComponent};
use std::marker::PhantomData;

pub struct JsonbChain<'a, T> {
    pub(crate) base: Operand<'a, Postgres>,
    pub(crate) ops: Vec<JsonOp<'a>>,
    pub(crate) _marker: PhantomData<T>,
}

// JsonbChain also an Operand
impl<'a, T, X> IntoOperand<'a, T, Postgres, IsAst, X> for JsonbChain<'a, T> {
    fn into_operand(self) -> Operand<'a, Postgres> {
        self.to_operand()
    }
}

impl<'a, T> JsonbChain<'a, T> {
    #[inline(always)]
    fn to_operand(self) -> Operand<'a, Postgres> {
        // JsonChain is a function node that produces a value
        let func = ExtFunction::JsonChain { base: Box::new(self.base), ops: self.ops };
        Operand::from_function(Function::from_ext(func))
    }

    // --- Concatenation method: append Vec directly and return Self ---
    pub fn json_path<V, K, X>(mut self, key: V) -> Self
        where
            V: IntoOperand<'a, String, Postgres, K, X>, {
        self.ops.push(JsonOp::Path(key.into_operand()));
        self
    }

    pub fn json_path_array<V, K, X>(mut self, path: V) -> Self
        where
            V: IntoOperand<'a, Vec<String>, Postgres, K, X>, {
        self.ops.push(JsonOp::PathArray(path.into_operand()));
        self
    }

    pub fn json_concat<V, K, X>(mut self, val: V) -> Self
        where
            V: IntoOperand<'a, T, Postgres, K, X>, {
        self.ops.push(JsonOp::Concat(val.into_operand()));
        self
    }

    pub fn json_delete<V, K, X>(mut self, key: V) -> Self
        where
            V: IntoOperand<'a, String, Postgres, K, X>, {
        self.ops.push(JsonOp::Delete(key.into_operand()));
        self
    }

    pub fn json_delete_path<V, K, X>(mut self, path: V) -> Self
        where
            V: IntoOperand<'a, Vec<String>, Postgres, K, X>, {
        self.ops.push(JsonOp::DeletePath(path.into_operand()));
        self
    }

    // --- State transition: Switch to the Text extraction endpoint ---
    pub fn json_path_text<V, K, X>(mut self, key: V) -> JsonbText<'a>
        where
            V: IntoOperand<'a, String, Postgres, K, X>, {
        self.ops.push(JsonOp::PathText(key.into_operand()));
        JsonbText { base: self.base, ops: self.ops }
    }

    pub fn json_path_text_array<V, K, X>(mut self, path: V) -> JsonbText<'a>
        where
            V: IntoOperand<'a, Vec<String>, Postgres, K, X>, {
        self.ops.push(JsonOp::PathTextArray(path.into_operand()));
        JsonbText { base: self.base, ops: self.ops }
    }

    // --- JSONB-specific Boolean endpoint methods ---
    pub fn json_contains<V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, T, Postgres, K, X>, {
        Condition::from(Operator::from_ext(ExtOperator::JsonContains(
            // IntoOperand::<'a, T, Postgres, K, X>::into_operand(self),
            self.to_operand(),
            val.into_operand(),
        )))
    }

    pub fn json_contained_by<V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, T, Postgres, K, X>, {
        Condition::from(Operator::from_ext(ExtOperator::JsonContainedBy(
            self.to_operand(),
            val.into_operand(),
        )))
    }

    pub fn json_has_key<V, K, X>(self, key: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>, {
        Condition::from(Operator::from_ext(ExtOperator::JsonHasKey(
            self.to_operand(),
            key.into_operand(),
        )))
    }

    pub fn json_has_any_keys<V, K, X>(self, keys: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, Vec<String>, Postgres, K, X>, {
        Condition::from(Operator::from_ext(ExtOperator::JsonHasAnyKeys(
            self.to_operand(),
            keys.into_operand(),
        )))
    }

    pub fn json_has_all_keys<V, K, X>(self, keys: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, Vec<String>, Postgres, K, X>, {
        Condition::from(Operator::from_ext(ExtOperator::JsonHasAllKeys(
            self.to_operand(),
            keys.into_operand(),
        )))
    }

    pub fn json_path_exists<V, K, X>(self, path: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>, {
        Condition::from(Operator::from_ext(ExtOperator::JsonPathExists(
            self.to_operand(),
            path.into_operand(),
        )))
    }

    pub fn json_path_predicate<V, K, X>(self, path: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X>, {
        Condition::from(Operator::from_ext(ExtOperator::JsonPathPredicate(
            self.to_operand(),
            path.into_operand(),
        )))
    }
}
