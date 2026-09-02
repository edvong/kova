use crate::dsl::arg::SingleArgMarker;
use crate::dsl::component::WithComponent;
use crate::dsl::condition::{Condition, ConditionBuilder};
use crate::dsl::operand::IntoOperand;
use crate::dsl::operator::{Operator, StandardOperator};
use crate::dsl::{Assignment, Operand, OrderOperand, StandardAssignment};
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

impl<'a, T, DB: Database> SingleArgMarker for Column<'a, T, DB> {}


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

impl<'a, T, DB: Database> ConditionBuilder<'a, DB> for &Column<'a, T, DB> {
    fn build_operand(self) -> Operand<'a, DB> {
        Operand::Column(self.meta)
    }
}

impl<'a, T, DB: Database> Column<'a, T, DB> {
    pub fn eq<V, M>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Eq)
    }

    pub fn ne<V, M>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Ne)
    }

    pub fn gt<V, M>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Gt)
    }

    pub fn ge<V, M>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Ge)
    }

    pub fn lt<V, M>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Lt)
    }

    pub fn le<V, M>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        self.binary_op(v, StandardOperator::Le)
    }

    pub fn is_null(&self) -> Condition<'a, DB> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn is_not_null(&self) -> Condition<'a, DB> {
        self.unary_op(StandardOperator::IsNotNull)
    }

    pub fn between<L, M1, U, M2>(&self, lower: L, upper: U) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, M1, DB>,
        U: IntoOperand<'a, M2, DB>,
    {
        self.ternary_op(lower, upper, StandardOperator::Between)
    }

    pub fn not_between<L, M1, U, M2>(&self, lower: L, upper: U) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, M1, DB>,
        U: IntoOperand<'a, M2, DB>,
    {
        self.ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    pub fn in_list<I, V, M>(&self, values: I) -> Condition<'a, DB>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, M, DB>,
    {
        self.list_op(values, StandardOperator::In)
    }

    pub fn not_in<I, V, M>(&self, values: I) -> Condition<'a, DB>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, M, DB>,
    {
        self.list_op(values, StandardOperator::NotIn)
    }

    pub fn like<V, M>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Like(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    pub fn not_like<V, M>(&self, v: V) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
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

    pub fn set<V, M>(&self, v: V) -> Assignment<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
    {
        Assignment::from_std(StandardAssignment::Value(self.meta, v.into_operand()))
    }
}
