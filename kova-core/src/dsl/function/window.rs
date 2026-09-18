#![allow(clippy::wrong_self_convention)]

use crate::dsl::function::{Function, IntoFunction, StandardFunction};
use crate::dsl::{
    BuildOrdering, BuildProjection, Comparator, Condition, Direction, EraseX, IntoArg, IntoArgs,
    IntoComparator, IntoIdentifier, IntoOperand, Operand, OrderOperand, Ordering, Projection,
    ProjectionExpr, SingleArgMarker, StandardOperand, WithComponent,
};
use crate::types::Database;
use std::marker::PhantomData;

pub struct Window<'a, DB: Database, X> {
    pub(crate) func: Function<'a, DB>,
    pub(crate) partition_by: Vec<Operand<'a, DB>>,
    pub(crate) order_by: Vec<Ordering<'a, DB, ()>>,
    pub(crate) _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> BuildProjection<'a, DB, X> for Window<'a, DB, X> {
    #[inline(always)]
    fn build_projection<A, U>(self, alias: A) -> Projection<'a, DB, X>
        where
            A: IntoIdentifier<U> {
        Projection::new(
            ProjectionExpr::Function(self.into_func()),
            Some(alias.into_identifier().meta),
        )
    }
}

impl<'a, DB: Database, X> IntoFunction<'a, DB> for Window<'a, DB, X> {
    #[inline(always)]
    fn into_func(self) -> Function<'a, DB> {
        Function::from_std(StandardFunction::Window(Box::new(self.erase_x())))
    }
}

impl<'a, DB: Database, X> EraseX for Window<'a, DB, X> {
    type Erased = Window<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Window {
            func: self.func,
            partition_by: self.partition_by,
            order_by: self.order_by,
            _marker: PhantomData,
        }
    }
}

impl<'a, DB: Database, X> SingleArgMarker for Window<'a, DB, X> {}

impl<'a, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Window<'a, DB, X> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, DB, X> {
        Projection::new(ProjectionExpr::Function(self.into_func()), None)
    }
}

impl<'a, Target, DB: Database, X> IntoComparator<'a, Target, DB> for Window<'a, DB, X> {
    #[inline(always)]
    fn into_comparator(self) -> Comparator<'a, Target, DB> {
        Comparator::new(Operand::from_function(self.into_func()))
    }
}

impl<'a, DB: Database, X> BuildOrdering<'a, DB, X> for Window<'a, DB, X> {
    #[inline(always)]
    fn build_ordering(self, direction: Option<Direction>) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(self.into_func())),
            direction,
            nulls: None,
            _marker: PhantomData,
        }
    }
}

impl<'a, DB: Database, X> Window<'a, DB, X> {
    pub fn partition_by<C, K>(mut self, cols: C) -> Self
        where
            C: IntoArgs<Operand<'a, DB>> {
        self.partition_by = cols.into_args();
        self
    }

    pub fn order_by<C>(mut self, cols: C) -> Self
        where
            C: IntoArgs<Ordering<'a, DB, X>> {
        self.order_by = cols.into_mapped_args(|o| o.erase_x());
        self
    }

    pub fn alias<A: IntoIdentifier<T>, T>(self, alias: A) -> Projection<'a, DB, X> {
        self.build_projection(alias)
    }

    pub fn asc(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Asc))
    }

    pub fn desc(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Desc))
    }
}

impl<'a, DB: Database, X> Window<'a, DB, X> {
    pub fn eq<V, T, K>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().eq(v)
    }

    pub fn ne<V, T, K>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().ne(v)
    }

    pub fn gt<V, T, K>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().gt(v)
    }

    pub fn ge<V, T, K>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().ge(v)
    }

    pub fn lt<V, T, K>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().lt(v)
    }

    pub fn le<V, T, K>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().le(v)
    }

    pub fn is_null<T>(self) -> Condition<'a, DB, X> {
        IntoComparator::<'a, T, DB>::into_comparator(self).is_null()
    }

    pub fn is_not_null<T>(self) -> Condition<'a, DB, X> {
        IntoComparator::<'a, T, DB>::into_comparator(self).is_not_null()
    }

    pub fn between<L, U, T, K>(self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, T, DB, K, X>,
            U: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().between(lower, upper)
    }

    pub fn not_between<L, U, T, K>(self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, T, DB, K, X>,
            U: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().not_between(lower, upper)
    }

    pub fn in_list<I, V, T, K>(self, values: I) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().in_list(values)
    }

    pub fn not_in<I, V, T, K>(self, values: I) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().not_in_list(values)
    }
}
