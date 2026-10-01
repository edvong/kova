use crate::dsl::jsonb_chain::JsonbChain;
use crate::dsl::jsonb_text::JsonbText;
use crate::dsl::operator::JsonOp;
use crate::dsl::{ExtFunction, ExtOperator};
use crate::types::Postgres;
use kova_core::dsl::{
    Condition, Identifier, IntoIdentifier, IntoOperand, Operand, Operator, WithComponent,
};
use kova_core::types::{IdentifierMeta, SyncMeta};
use std::marker::PhantomData;

pub struct ExtVirtualColumn<T = ()> {
    pub meta: IdentifierMeta,
    _marker: PhantomData<T>,
}

impl<T> Copy for ExtVirtualColumn<T> {}
impl<T> Clone for ExtVirtualColumn<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> SyncMeta<IdentifierMeta> for ExtVirtualColumn<T> {
    fn sync(&mut self, meta: IdentifierMeta) {
        self.meta = meta;
    }
}

impl<T> IntoIdentifier<T> for ExtVirtualColumn<T> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta)
    }
}

impl<T> IntoIdentifier<T> for &ExtVirtualColumn<T> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta)
    }
}

impl<T> ExtVirtualColumn<T> {
    #[doc(hidden)]
    pub const fn new(meta: IdentifierMeta) -> ExtVirtualColumn<T> {
        ExtVirtualColumn { meta, _marker: PhantomData }
    }

    #[inline(always)]
    fn op<'a>(self) -> Operand<'a, Postgres> {
        Operand::from_identifier(self.into_identifier().meta)
    }

    // =====================================================
    // Start chaining state
    // (allocate and initialize a single-element Vec)
    // =====================================================

    pub fn json_path<'a, V, K, X>(self, key: V) -> JsonbChain<'a, T, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        JsonbChain {
            base: self.op(),
            ops: vec![JsonOp::Path(key.into_operand())],
            _marker: PhantomData,
        }
    }

    pub fn json_path_array<'a, I, V, K, X>(self, path: I) -> JsonbChain<'a, T, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X> {
        JsonbChain {
            base: self.op(),
            ops: vec![JsonOp::PathArray(ExtFunction::from_iter(path))],
            _marker: PhantomData,
        }
    }

    pub fn json_concat<'a, V, K, X>(self, val: V) -> JsonbChain<'a, T, X>
        where
            V: IntoOperand<'a, T, Postgres, K, X> {
        JsonbChain {
            base: self.op(),
            ops: vec![JsonOp::Concat(val.into_operand())],
            _marker: PhantomData,
        }
    }

    pub fn json_delete<'a, V, K, X>(self, key: V) -> JsonbChain<'a, T, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        JsonbChain {
            base: self.op(),
            ops: vec![JsonOp::Delete(key.into_operand())],
            _marker: PhantomData,
        }
    }

    pub fn json_delete_path<'a, I, V, K, X>(self, path: I) -> JsonbChain<'a, T, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X> {
        JsonbChain {
            base: self.op(),
            ops: vec![JsonOp::DeletePath(ExtFunction::from_iter(path))],
            _marker: PhantomData,
        }
    }

    // =====================================================
    // Toggles text extraction endpoints on startup
    // =====================================================

    pub fn json_path_text<'a, V, K, X>(self, key: V) -> JsonbText<'a, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        JsonbText {
            base: self.op(),
            ops: vec![JsonOp::PathText(key.into_operand())],
            _marker: PhantomData,
        }
    }

    pub fn json_path_text_array<'a, I, V, K, X>(self, path: I) -> JsonbText<'a, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, String, Postgres, K, X> {
        JsonbText {
            base: self.op(),
            ops: vec![JsonOp::PathTextArray(ExtFunction::from_iter(path))],
            _marker: PhantomData,
        }
    }

    // =====================================================
    // Start and directly complete condition judgment
    // =====================================================

    pub fn json_contains<'a, V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, T, Postgres, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::JsonContains(
            self.op(),
            val.into_operand(),
        )))
    }

    pub fn json_contained_by<'a, V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, T, Postgres, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::JsonContainedBy(
            self.op(),
            val.into_operand(),
        )))
    }

    pub fn json_has_key<'a, V, K, X>(self, key: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::JsonHasKey(self.op(), key.into_operand())))
    }

    pub fn json_has_any_keys<'a, I, V, K, X>(self, keys: I) -> Condition<'a, Postgres, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Vec<String>, Postgres, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::JsonHasAnyKeys(
            self.op(),
            ExtFunction::from_iter(keys),
        )))
    }

    pub fn json_has_all_keys<'a, V, K, X>(self, keys: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, Vec<String>, Postgres, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::JsonHasAllKeys(
            self.op(),
            keys.into_operand(),
        )))
    }

    pub fn json_path_exists<'a, V, K, X>(self, path: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::JsonPathExists(
            self.op(),
            path.into_operand(),
        )))
    }

    pub fn json_path_predicate<'a, V, K, X>(self, path: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, String, Postgres, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::JsonPathPredicate(
            self.op(),
            path.into_operand(),
        )))
    }
}
