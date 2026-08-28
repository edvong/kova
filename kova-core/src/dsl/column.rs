use crate::dsl::arg::SingleArgMarker;
use crate::dsl::component::WithComponent;
use crate::dsl::condition::Condition;
use crate::dsl::operand::IntoOperand;
use crate::dsl::operator::{Operator, StandardOperator};
use crate::dsl::{Operand, OrderOperand};

use crate::dsl::{Direction, Ordering, Projection, ProjectionExpr, StandardOperand};
use crate::types::{ColumnMeta, Database, Extension};
use std::ops::Deref;

pub type ExtColumn<'a, T, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Column<T>;

pub struct Column<'a, T, DB: Database> {
    pub(crate) meta: ColumnMeta,
    inner: ExtColumn<'a, T, DB>,
}

impl<'a, T, DB: Database> Deref for Column<'a, T, DB> {
    type Target = ExtColumn<'a, T, DB>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// impl<'a, DB: Database> From<StandardOperator<'a, DB>> for Operator<'a, DB> {
//     fn from(value: StandardOperator<'a, DB>) -> Self {
//         Operator::from_std(value)
//     }
// }

impl<'a, T, DB: Database> SingleArgMarker for Column<'a, T, DB> {}

// ==== Projection START ====

// Projection
impl<'a, T, DB: Database> Column<'a, T, DB> {
    /// The type of alias T must be consistent with the current column
    pub fn alias(self, alias: Column<'a, T, DB>) -> Projection<'a, DB> {
        Projection {
            expr: ProjectionExpr::Column(self.meta),
            alias: Some(alias.meta),
        }
    }
}

/// Unconstrained impl blocks, specialized for `const fn` constructors
impl<'a, T, DB: Database> Column<'a, T, DB> {
    pub const fn new(meta: ColumnMeta, ext: ExtColumn<'a, T, DB>) -> Self {
        Self { meta, inner: ext }
    }
    pub fn meta(&self) -> ColumnMeta {
        self.meta
    }
}

impl<'a, T, DB: Database> Column<'a, T, DB> {
    pub fn eq<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        // Operator::from_std(value)
        Condition::Op(Operator::from_std(StandardOperator::Eq(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn ne<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Ne(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn gt<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Gt(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn ge<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Ge(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn lt<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Lt(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn le<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Le(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn is_null(&self) -> Condition<'a, DB> {
        Condition::Op(Operator::from_std(StandardOperator::IsNull(
            Operand::Column(self.meta),
        )))
    }

    pub fn is_not_null(&self) -> Condition<'a, DB> {
        Condition::Op(Operator::from_std(StandardOperator::IsNotNull(
            Operand::Column(self.meta),
        )))
    }

    pub fn between<L, U>(&self, lower: L, upper: U) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, L, DB>,
        U: IntoOperand<'a, U, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Between(
            Operand::Column(self.meta),
            lower.into_operand(),
            upper.into_operand(),
        )))
    }

    pub fn not_between<L, U>(&self, lower: L, upper: U) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, L, DB>,
        U: IntoOperand<'a, U, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::NotBetween(
            Operand::Column(self.meta),
            lower.into_operand(),
            upper.into_operand(),
        )))
    }

    pub fn in_list<I, V>(&self, values: I) -> Condition<'a, DB>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, V, DB>,
    {
        let values = values
            .into_iter()
            .map(|value| value.into_operand())
            .collect();
        Condition::Op(Operator::from_std(StandardOperator::In(
            Operand::Column(self.meta),
            values,
        )))
    }

    pub fn not_in<I, V>(&self, values: I) -> Condition<'a, DB>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, V, DB>,
    {
        let values = values
            .into_iter()
            .map(|value| value.into_operand())
            .collect();
        Condition::Op(Operator::from_std(StandardOperator::NotIn(
            Operand::Column(self.meta),
            values,
        )))
    }

    pub fn like<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Like(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn not_like<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::NotLike(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn asc(&self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Column(self.meta)),
            direction: Some(Direction::Asc),
            nulls: None,
        }
    }

    pub fn desc(&self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Column(self.meta)),
            direction: Some(Direction::Desc),
            nulls: None,
        }
    }

    // pub fn set<V>(&self, value: V) -> Assignment<'a, DB> where V: IntoOperand<'a, V, DB> {
    //     Assignment::Value(Operand::Column(self.meta), value.into_operand())
    // }
}
