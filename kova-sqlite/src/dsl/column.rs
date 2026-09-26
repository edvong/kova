use crate::dsl::ExtOperator;
use crate::dsl::assignment::ExtAssignment;
use crate::dsl::function::JsonbExtract;
use crate::dsl::function::{Bm25, Highlight, JsonExtract, Snippet};
use crate::types::Sqlite;
use kova_core::dsl::{
    Assignment, Column, Condition, IntoOperand, Operand, Operator, WithComponent,
};
use kova_core::types::{IdentifierMeta, SyncIdentifierMeta};
use std::marker::PhantomData;

pub struct ExtColumn<T> {
    meta: IdentifierMeta,
    _marker: PhantomData<T>,
}

impl<T> SyncIdentifierMeta for ExtColumn<T> {
    fn sync(&mut self, meta: IdentifierMeta) {
        self.meta = meta;
    }
}

impl<T> ExtColumn<T> {
    #[doc(hidden)]
    pub const fn create<'a>(meta: IdentifierMeta) -> Column<'a, T, Sqlite> {
        Column::new(meta, Self::new(meta))
    }

    const fn new(meta: IdentifierMeta) -> ExtColumn<T> {
        Self { meta, _marker: PhantomData }
    }
}

impl<T> ExtColumn<T> {
    pub fn excluded<'a, X>(&self) -> Assignment<'a, Sqlite, X> {
        Assignment::from_ext(ExtAssignment::ExcludedColumn(self.meta))
    }

    #[inline(always)]
    fn op<'a>(&self) -> Operand<'a, Sqlite> {
        Operand::from_identifier(self.meta)
    }

    // json_extract function
    pub fn json_extract<'a, V, K, X>(&self, path: V) -> JsonExtract<'a, X>
        where
            V: IntoOperand<'a, String, Sqlite, K, X> {
        JsonExtract { base: self.op(), path: path.into_operand(), _marker: PhantomData }
    }

    // jsonb_extract function
    pub fn jsonb_extract<'a, V, K, X>(&self, path: V) -> JsonbExtract<'a, X>
        where
            V: IntoOperand<'a, String, Sqlite, K, X> {
        JsonbExtract { base: self.op(), path: path.into_operand(), _marker: PhantomData }
    }

    // FTS5 Full-text search MATCH operator
    pub fn matches<'a, V, K, X>(&self, pattern: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, String, Sqlite, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::Match(self.op(), pattern.into_operand())))
    }

    // Regular Expressions
    pub fn regexp<'a, V, K, X>(&self, pattern: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, String, Sqlite, K, X> {
        Condition::from(Operator::from_ext(ExtOperator::Regexp(self.op(), pattern.into_operand())))
    }

    pub fn bm25<'a, X>(&self) -> Bm25<'a, X> {
        Bm25 { table: self.meta.source, _marker: PhantomData }
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
