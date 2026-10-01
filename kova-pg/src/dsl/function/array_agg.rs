#![allow(clippy::wrong_self_convention)]

use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{Column, Comparator, Condition, EraseX, IntoComparator, IntoIdentifier, IntoOperand, OpAst, Operand, Ordering, Projection, ProjectionExpr, WithComponent};
use std::marker::PhantomData;

pub struct ArrayAggState<'a, X> {
    pub(crate) target: Box<Operand<'a, Postgres>>,
    pub(crate) distinct: bool,
    pub(crate) order_by: Vec<Ordering<'a, Postgres, ()>>,
    pub(crate) filter: Option<Box<Condition<'a, Postgres, ()>>>,
    pub(crate) _marker: PhantomData<X>,
}

impl<'a, X> EraseX for ArrayAggState<'a, X> {
    type Erased = ArrayAggState<'a, ()>;

    fn erase_x(self) -> Self::Erased {
        ArrayAggState {
            target: self.target,
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

impl<'a, X> IntoArrayAggState<'a> for ArrayAggState<'a, X> {
    type X = X;
    fn into_state(self) -> ArrayAggState<'a, X> {
        self
    }
}

impl<'a, U> IntoArrayAggState<'a> for Column<U, Postgres> {
    type X = ();
    fn into_state(self) -> ArrayAggState<'a, ()> {
        ArrayAggState {
            target: Box::new(IntoOperand::<U, Postgres, OpAst, Self::X>::into_operand(self)),
            distinct: false,
            order_by: vec![],
            filter: None,
            _marker: PhantomData,
        }
    }
}


macro_rules! impl_func_array_agg_state {
    ($($func:ident),+) => {
        $(
            impl<'a, C> IntoArrayAggState<'a> for kova_core::dsl::func::$func<C>
                where
                    Self: kova_core::dsl::OpNode + IntoOperand<'a, <Self as kova_core::dsl::OpNode>::Output, Postgres, OpAst, C::X>,
                    C: kova_core::dsl::OpNode + IntoArrayAggState<'a> { // Allow nesting: ArrayAgg(Lower(Upper(col)))
                type X = C::X; // Upward penetration
                #[inline(always)]
                fn into_state(self) -> ArrayAggState<'a, Self::X> {
                    ArrayAggState {
                        // use function self IntoOperand
                        target: Box::new(self.into_operand()),
                        distinct: false,
                        order_by: vec![],
                        filter: None,
                        _marker: std::marker::PhantomData,
                    }
                }
            }
        )+
    };
}

impl_func_array_agg_state!(Lower, Upper, Abs, Length); // Btrim, Ltrim, Rtrim, Md5



// ==========================================

pub struct ArrayAgg<T>(pub T);

impl<'a, T> ArrayAgg<T>
    where
        T: IntoArrayAggState<'a> {
    // --- State Transition (Keep the current T::X unchanged) ---

    pub fn distinct(self) -> ArrayAgg<ArrayAggState<'a, T::X>> {
        let mut state = self.0.into_state();
        state.distinct = true;
        ArrayAgg(state)
    }

    pub fn order_by(
        self, ordering: Ordering<'a, Postgres, T::X>,
    ) -> ArrayAgg<ArrayAggState<'a, T::X>> {
        let mut state = self.0.into_state();
        state.order_by.push(ordering.erase_x());
        ArrayAgg(state)
    }

    pub fn filter<X>(self, cond: Condition<'a, Postgres, X>) -> ArrayAgg<ArrayAggState<'a, X>> {
        let state = self.0.into_state();
        ArrayAgg(ArrayAggState {
            target: state.target,
            distinct: state.distinct,
            order_by: state.order_by,
            filter: Some(Box::new(cond.erase_x())),
            _marker: PhantomData,
        })
    }

    // --- Delivery (Extract T::X) ---

    pub fn alias<U>(self, col: Column<U, Postgres>) -> Projection<'a, Postgres, T::X> {
        Projection::new(
            ProjectionExpr::Function(self.build_func()),
            Some(col.into_identifier().meta),
        )
    }

    fn build_func(self) -> Function<'a, Postgres> {
        Function::from_ext(crate::dsl::ExtFunction::ArrayAgg(self.0.into_state().erase_x()))
    }
}

// ====================== Operators ======================

impl<'a, T, Target> IntoComparator<'a, Target, Postgres> for ArrayAgg<T>
    where
        T: IntoArrayAggState<'a> {
    #[inline(always)]
    fn into_comparator(self) -> Comparator<'a, Target, Postgres> {
        Comparator::new(Operand::from_function(Function::from_ext(ExtFunction::ArrayAgg(
            self.0.into_state().erase_x(),
        ))))
    }
}

impl<'a, T> ArrayAgg<T>
    where
        T: IntoArrayAggState<'a> {
    pub fn eq<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
        where
            V: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().eq(v)
    }

    pub fn ne<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
        where
            V: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().ne(v)
    }

    pub fn gt<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
        where
            V: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().gt(v)
    }

    pub fn ge<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
        where
            V: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().ge(v)
    }

    pub fn lt<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
        where
            V: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().lt(v)
    }

    pub fn le<V, Target, K>(self, v: V) -> Condition<'a, Postgres, T::X>
        where
            V: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().le(v)
    }

    pub fn is_null(self) -> Condition<'a, Postgres, T::X> {
        IntoComparator::<'a, (), Postgres>::into_comparator(self).is_null()
    }

    pub fn is_not_null(self) -> Condition<'a, Postgres, T::X> {
        IntoComparator::<'a, (), Postgres>::into_comparator(self).is_not_null()
    }

    pub fn between<L, U, Target, K>(self, lower: L, upper: U) -> Condition<'a, Postgres, T::X>
        where
            L: IntoOperand<'a, Target, Postgres, K, T::X>,
            U: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().between(lower, upper)
    }

    pub fn not_between<L, U, Target, K>(self, lower: L, upper: U) -> Condition<'a, Postgres, T::X>
        where
            L: IntoOperand<'a, Target, Postgres, K, T::X>,
            U: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().not_between(lower, upper)
    }

    pub fn in_list<I, V, Target, K>(self, values: I) -> Condition<'a, Postgres, T::X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().in_list(values)
    }

    pub fn not_in_list<I, V, Target, K>(self, values: I) -> Condition<'a, Postgres, T::X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, Postgres, K, T::X> {
        self.into_comparator().not_in_list(values)
    }

    // Other PG Operators ...
    // array_agg(x)
    // array_agg(ALL x)
    // array_agg(DISTINCT x)
    // array_agg(x ORDER BY y)
    // array_agg(DISTINCT x ORDER BY y)
    // array_agg(x) FILTER (WHERE ...)
    // array_agg(DISTINCT x ORDER BY y) FILTER (WHERE ...)
}
