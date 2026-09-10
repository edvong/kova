#![allow(clippy::wrong_self_convention)]

use crate::types::Postgres;
use kova_core::dsl::func::Function as CoreFunction;
use kova_core::dsl::{
    Column, Condition, ConditionBuilder, EraseX, IntoOperand, Operand, Ordering, Projection,
    ProjectionExpr, StandardOperator, WithComponent,
};
use kova_core::types::ColumnMeta;
use std::marker::PhantomData;

// array_agg(x)
// array_agg(ALL x)
// array_agg(DISTINCT x)
// array_agg(x ORDER BY y)
// array_agg(DISTINCT x ORDER BY y)
// array_agg(x) FILTER (WHERE ...)
// array_agg(DISTINCT x ORDER BY y) FILTER (WHERE ...)

pub struct ArrayAggState<'a, X> {
    pub(crate) meta: ColumnMeta,
    pub(crate) distinct: bool,
    pub(crate) order_by: Vec<Ordering<'a, Postgres, ()>>,
    pub(crate) filter: Option<Box<Condition<'a, Postgres, ()>>>,
    pub(crate) _marker: PhantomData<X>,
}

impl<'a, X> EraseX for ArrayAggState<'a, X> {
    type Erased = ArrayAggState<'a, ()>;

    fn erase_x(self) -> Self::Erased {
        ArrayAggState {
            meta: self.meta,
            distinct: self.distinct,
            order_by: self.order_by,
            filter: self.filter,
            _marker: PhantomData,
        }
    }
}

pub trait IntoArrayAggState<'a> {
    type X;
    fn into_state(self) -> ArrayAggState<'a, Self::X>;
}

impl<'a, U> IntoArrayAggState<'a> for Column<'a, U, Postgres> {
    type X = ();
    fn into_state(self) -> ArrayAggState<'a, ()> {
        ArrayAggState {
            meta: self.meta(),
            distinct: false,
            order_by: vec![],
            filter: None,
            _marker: PhantomData,
        }
    }
}

impl<'a, X> IntoArrayAggState<'a> for ArrayAggState<'a, X> {
    type X = X;
    fn into_state(self) -> ArrayAggState<'a, X> {
        self
    }
}

// ==========================================

pub struct ArrayAgg<T>(pub T);

impl<'a, T> ArrayAgg<T>
where
    T: IntoArrayAggState<'a>,
{
    // --- State Transition (Keep the current T::X unchanged) ---

    pub fn distinct(self) -> ArrayAgg<ArrayAggState<'a, T::X>> {
        let mut state = self.0.into_state();
        state.distinct = true;
        ArrayAgg(state)
    }

    pub fn order_by(
        self,
        ordering: Ordering<'a, Postgres, T::X>,
    ) -> ArrayAgg<ArrayAggState<'a, T::X>> {
        let mut state = self.0.into_state();
        state.order_by.push(ordering.erase_x());
        ArrayAgg(state)
    }

    pub fn filter<NewX>(
        self,
        cond: Condition<'a, Postgres, NewX>,
    ) -> ArrayAgg<ArrayAggState<'a, NewX>> {
        let state = self.0.into_state();
        ArrayAgg(ArrayAggState {
            meta: state.meta,
            distinct: state.distinct,
            order_by: state.order_by,
            filter: Some(Box::new(cond.erase_x())),
            _marker: PhantomData,
        })
    }

    // --- Delivery (Extract T::X) ---

    pub fn alias<U>(self, col: Column<'a, U, Postgres>) -> Projection<'a, Postgres, T::X> {
        Projection::new(
            ProjectionExpr::Function(self.build_func()),
            Some(col.meta()),
        )
    }

    fn build_func(self) -> CoreFunction<'a, Postgres> {
        CoreFunction::from_ext(crate::dsl::Function::ArrayAgg(
            self.0.into_state().erase_x(),
        ))
    }
}

// ====================== Operators ======================

impl<'a, T> ConditionBuilder<'a, Postgres> for ArrayAgg<T>
where
    T: IntoArrayAggState<'a>,
{
    #[inline(always)]
    fn build_operand(self) -> Operand<'a, Postgres> {
        Operand::Function(self.build_func())
    }
}

impl<'a, T> ArrayAgg<T>
where
    T: IntoArrayAggState<'a>,
{
    pub fn eq<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
    where
        V: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.binary_op(v, StandardOperator::Eq)
    }

    pub fn ne<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
    where
        V: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.binary_op(v, StandardOperator::Ne)
    }

    pub fn gt<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
    where
        V: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.binary_op(v, StandardOperator::Gt)
    }

    pub fn ge<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
    where
        V: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.binary_op(v, StandardOperator::Ge)
    }

    pub fn lt<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
    where
        V: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.binary_op(v, StandardOperator::Lt)
    }

    pub fn le<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
    where
        V: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.binary_op(v, StandardOperator::Le)
    }

    pub fn is_null(self) -> Condition<'a, Postgres, T::X> {
        self.unary_op(StandardOperator::IsNull)
    }

    pub fn is_not_null(self) -> Condition<'a, Postgres, T::X> {
        self.unary_op(StandardOperator::IsNotNull)
    }

    pub fn between<L, U, Target, K>(self, lower: L, upper: U) -> Condition<'a, Postgres, T::X>
    where
        L: IntoOperand<'a, Target, Postgres, K, T::X>,
        U: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.ternary_op(lower, upper, StandardOperator::Between)
    }

    pub fn not_between<L, U, Target, K>(self, lower: L, upper: U) -> Condition<'a, Postgres, T::X>
    where
        L: IntoOperand<'a, Target, Postgres, K, T::X>,
        U: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    pub fn in_list<I, V, Target, K>(self, values: I) -> Condition<'a, Postgres, T::X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.list_op(values, StandardOperator::In)
    }

    pub fn not_in<I, V, Target, K>(self, values: I) -> Condition<'a, Postgres, T::X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, Postgres, K, T::X>,
    {
        self.list_op(values, StandardOperator::NotIn)
    }

    // Other PG Operators ...
}
