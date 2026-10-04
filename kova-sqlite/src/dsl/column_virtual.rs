use crate::dsl::ExtOperator;
use crate::dsl::func::{Bm25, Highlight, JsonExtract, JsonbExtract, Snippet};
use crate::types::Sqlite;
use kova_core::dsl::{Condition, Identifier, IntoIdentifier, IntoOperand, Operand, Operator};
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
    fn op<'a>(&self) -> Operand<'a, Sqlite> {
        Operand::from_column(self.meta)
    }

    // json_extract function
    pub fn json_extract<'a, V, K, X>(&self, path: V) -> JsonExtract<'a, X>
        where
            V: IntoOperand<'a, String, Sqlite, K, X> {
        JsonExtract::new(self.op(), path.into_operand())
        // JsonExtract { base: self.op(), path: path.into_operand(), _marker: PhantomData }
    }

    // jsonb_extract function
    pub fn jsonb_extract<'a, V, K, X>(&self, path: V) -> JsonbExtract<'a, X>
        where
            V: IntoOperand<'a, String, Sqlite, K, X> {
        JsonbExtract::new(self.op(), path.into_operand())
    }

    // FTS5 Full-text search MATCH operator
    pub fn matches<'a, V, K, X>(&self, pattern: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, String, Sqlite, K, X> {
        Condition::from(Operator::from(ExtOperator::Match(self.op(), pattern.into_operand())))
    }

    // Regular Expressions
    pub fn regexp<'a, V, K, X>(&self, pattern: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, String, Sqlite, K, X> {
        Condition::from(Operator::from(ExtOperator::Regexp(self.op(), pattern.into_operand())))
    }

    pub fn bm25<'a, X>(&self) -> Bm25<'a, X> {
        Bm25::new(self.meta.source)
    }

    pub fn highlight<'a, L, R, K1, K2, X>(
        &self, col_index: i32, left: L, right: R,
    ) -> Highlight<'a, X>
        where
            L: IntoOperand<'a, String, Sqlite, K1, X>,
            R: IntoOperand<'a, String, Sqlite, K2, X> {
        Highlight::new(self.meta.source, col_index, left, right)
    }

    pub fn snippet<'a, L, R, E, K1, K2, K3, X>(
        &self, col_index: i32, left: L, right: R, ellipsis: E, max_tokens: i32,
    ) -> Snippet<'a, X>
        where
            L: IntoOperand<'a, String, Sqlite, K1, X>,
            R: IntoOperand<'a, String, Sqlite, K2, X>,
            E: IntoOperand<'a, String, Sqlite, K3, X> {
        Snippet::new(self.meta.source, col_index, left, right, ellipsis, max_tokens)
    }
}
