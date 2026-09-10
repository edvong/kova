use crate::dsl::arg::SingleArgMarker;
use crate::dsl::component::WithComponent;
use crate::dsl::condition::{Condition, ConditionBuilder, ConditionNode};
use crate::dsl::operand::IntoOperand;
use crate::dsl::operator::{Operator, StandardOperator};
use crate::dsl::{IsAst, Assignment, Operand, OrderOperand, StandardAssignment};
use crate::dsl::{Direction, Ordering, Projection, ProjectionExpr, StandardOperand};
use crate::types::{ColumnMeta, Database, Extension};
use std::marker::PhantomData;
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

impl<'a, T, DB: Database> SingleArgMarker for Column<'a, T, DB> {}

impl<'a, T, DB: Database> Column<'a, T, DB> {
    /// The type of alias T must be consistent with the current column
    pub fn alias<X>(self, alias: Column<'a, T, DB>) -> Projection<'a, DB, X> {
        Projection::new(ProjectionExpr::Column(self.meta), Some(alias.meta))
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

impl<'a, T, DB: Database> ConditionBuilder<'a, DB> for &Column<'a, T, DB> {
    fn build_operand(self) -> Operand<'a, DB> {
        Operand::Column(self.meta)
    }
}

impl<'a, T, DB: Database, X> IntoOperand<'a, T, DB, IsAst, X> for Column<'a, T, DB> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::Column(self.meta)
    }
}

impl<'a, T, DB: Database> Column<'a, T, DB> {
    pub fn eq<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Eq)
    }

    pub fn ne<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Ne)
    }

    pub fn gt<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Gt)
    }

    pub fn ge<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Ge)
    }

    pub fn lt<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Lt)
    }

    pub fn le<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        self.binary_op(v, StandardOperator::Le)
    }

    pub fn is_null<X>(&self) -> Condition<'a, DB, X> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn is_not_null<X>(&self) -> Condition<'a, DB, X> {
        self.unary_op(StandardOperator::IsNotNull)
    }

    pub fn between<L, U, K, X>(&self, lower: L, upper: U) -> Condition<'a, DB, X>
    where
        L: IntoOperand<'a, T, DB, K, X>,
        U: IntoOperand<'a, T, DB, K, X>,
    {
        self.ternary_op(lower, upper, StandardOperator::Between)
    }

    pub fn not_between<L, U, K, X>(&self, lower: L, upper: U) -> Condition<'a, DB, X>
    where
        L: IntoOperand<'a, T, DB, K, X>,
        U: IntoOperand<'a, T, DB, K, X>,
    {
        self.ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    pub fn in_list<I, V, K, X>(&self, values: I) -> Condition<'a, DB, X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, T, DB, K, X>,
    {
        self.list_op(values, StandardOperator::In)
    }

    pub fn not_in<I, V, K, X>(&self, values: I) -> Condition<'a, DB, X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, T, DB, K, X>,
    {
        self.list_op(values, StandardOperator::NotIn)
    }

    pub fn like<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        ConditionNode::Op(Operator::from_std(StandardOperator::Like(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
        .into()
    }

    pub fn not_like<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        ConditionNode::Op(Operator::from_std(StandardOperator::NotLike(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
        .into()
    }

    pub fn asc<X>(&self) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Column(self.meta)),
            direction: Some(Direction::Asc),
            nulls: None,
            _marker: PhantomData,
        }
    }

    pub fn desc<X>(&self) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Column(self.meta)),
            direction: Some(Direction::Desc),
            nulls: None,
            _marker: PhantomData,
        }
    }

    pub fn set<V, K, X>(&self, v: V) -> Assignment<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X>,
    {
        Assignment::from_std(StandardAssignment::Value(self.meta, v.into_operand()))
    }
}
