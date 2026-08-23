use crate::dsl::Component;
use crate::dsl::condition::Condition;
use crate::dsl::operator::{IntoOperand, Operator, StandardOperator};
use crate::dsl::order::{OrderBy, OrderDir};
use crate::dsl::projection::{IntoProjection, Projectable, Projection, StandardProjection};
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

impl<'a, DB: Database> From<StandardOperator<'a, DB>> for Operator<'a, DB> {
    fn from(value: StandardOperator<'a, DB>) -> Self {
        Operator::Standard(value)
    }
}

impl<'a, T, DB: Database> IntoProjection<'a, DB> for Column<'a, T, DB> {
    fn into_projection(self) -> Projection<'a, DB> {
        Component::Standard(StandardProjection::Column(self.meta, None))
    }
}

impl<'a, T, DB: Database> Projectable<'a, DB> for Column<'a, T, DB> {
    fn into_projections(self) -> Vec<Projection<'a, DB>> {
        vec![self.into_projection()]
    }
}

/// Unconstrained impl blocks, specialized for `const fn` constructors
impl<'a, T, DB: Database> Column<'a, T, DB> {
    pub const fn new(meta: ColumnMeta, ext: ExtColumn<'a, T, DB>) -> Self {
        Self { meta, inner: ext }
    }
}

impl<'a, T, DB: Database> Column<'a, T, DB> {
    /// The type of alias T must be consistent with the current column
    pub fn alias(self, alias: Column<'a, T, DB>) -> Projection<'a, DB> {
        Component::Standard(StandardProjection::Column(self.meta, Some(alias.meta)))
    }
}

impl<'a, T, DB: Database> Column<'a, T, DB> {
    pub fn eq<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(StandardOperator::Eq(self.meta, v.into_operand()).into())
    }

    pub fn ne<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(StandardOperator::Ne(self.meta, v.into_operand()).into())
    }

    pub fn gt<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(StandardOperator::Gt(self.meta, v.into_operand()).into())
    }

    pub fn ge<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(StandardOperator::Ge(self.meta, v.into_operand()).into())
    }

    pub fn lt<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(StandardOperator::Lt(self.meta, v.into_operand()).into())
    }

    pub fn le<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(StandardOperator::Le(self.meta, v.into_operand()).into())
    }

    pub fn is_null(&self) -> Condition<'a, DB> {
        Condition::Op(StandardOperator::IsNull(self.meta).into())
    }

    pub fn is_not_null(&self) -> Condition<'a, DB> {
        Condition::Op(StandardOperator::IsNotNull(self.meta).into())
    }

    pub fn between<L, U>(&self, lower: L, upper: U) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, L, DB>,
        U: IntoOperand<'a, U, DB>,
    {
        Condition::Op(
            StandardOperator::Between(self.meta, lower.into_operand(), upper.into_operand()).into(),
        )
    }

    pub fn not_between<L, U>(&self, lower: L, upper: U) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, L, DB>,
        U: IntoOperand<'a, U, DB>,
    {
        Condition::Op(
            StandardOperator::NotBetween(self.meta, lower.into_operand(), upper.into_operand())
                .into(),
        )
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
        Condition::Op(StandardOperator::In(self.meta, values).into())
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
        Condition::Op(StandardOperator::NotIn(self.meta, values).into())
    }

    pub fn like<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(StandardOperator::Like(self.meta, v.into_operand()).into())
    }

    pub fn not_like<V>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, V, DB>,
    {
        Condition::Op(StandardOperator::NotLike(self.meta, v.into_operand()).into())
    }

    pub fn asc(&self) -> OrderBy {
        OrderBy {
            column: self.meta,
            direction: OrderDir::Asc,
        }
    }

    pub fn desc(&self) -> OrderBy {
        OrderBy {
            column: self.meta,
            direction: OrderDir::Desc,
        }
    }

    // pub fn set<V>(&self, value: V) -> Assignment<'a, DB> where V: IntoOperand<'a, V, DB> {
    //     Assignment::Value(self.meta, value.into_operand())
    // }
}
