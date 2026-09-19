use crate::dsl::ExtOperator;
use crate::dsl::assignment::ExtAssignment;
use crate::dsl::function::JsonExtract;
use crate::dsl::function::JsonbExtract;
use crate::types::Sqlite;
use kova_core::dsl::{
    Assignment, Column, Condition, IntoOperand, Operand, Operator, WithComponent,
};
use kova_core::types::IdentifierMeta;
use std::marker::PhantomData;

pub struct ExtColumn<T> {
    meta: IdentifierMeta,
    _marker: PhantomData<T>,
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
}
