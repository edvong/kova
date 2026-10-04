#![allow(clippy::wrong_self_convention)]

use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    Condition, EraseX, IntoArg, IntoArgs, IntoComparator, IntoIdentifier, IntoOperand,
    IntoProjection, OpAst, OpNode, Operand, Ordering, Projection, SingleArgMarker,
};

pub struct ArrayAgg<'a, T> {
    pub(crate) target: T,
    pub(crate) distinct: bool,
    pub(crate) order_by: Vec<Ordering<'a, Postgres, ()>>,
    pub(crate) filter: Option<Condition<'a, Postgres, ()>>,
}

#[allow(non_snake_case)]
pub const fn ArrayAgg<'a, T>(target: T) -> ArrayAgg<'a, T> {
    ArrayAgg { target, distinct: false, order_by: Vec::new(), filter: None }
}

impl<'a, T: OpNode> OpNode for ArrayAgg<'a, T> {
    type Input = T::Output;
    type Output = Vec<T::Output>;
}

impl<'a, T> ArrayAgg<'a, T> {
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    pub fn order_by<X>(mut self, ordering: impl IntoArgs<Ordering<'a, Postgres, X>>) -> Self {
        self.order_by = ordering.into_mapped_args(|o| o.erase_x());
        self
    }

    pub fn filter<X>(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.filter = Some(cond.erase_x());
        self
    }

    pub fn alias<A, U, X>(self, a: A) -> Projection<'a, Postgres, X>
        where
            A: IntoIdentifier<U>,
            Self: IntoProjection<'a, Postgres, X> {
        self.into_projection().alias(a)
    }
}

impl<'a, T> SingleArgMarker for ArrayAgg<'a, T> {}

impl<'a, Target, T, X> IntoOperand<'a, Target, Postgres, OpAst, X> for ArrayAgg<'a, T>
    where
        T: OpNode + IntoOperand<'a, <T as OpNode>::Output, Postgres, OpAst, X> {
    fn into_operand(self) -> Operand<'a, Postgres> {
        Operand::from_function(Function::from(ExtFunction::ArrayAgg {
            operand: Box::new(self.target.into_operand()),
            distinct: self.distinct,
            order_by: self.order_by,
            filter: self.filter,
        }))
    }
}

impl<'a, T, X> IntoProjection<'a, Postgres, X> for ArrayAgg<'a, T>
    where
        Self: IntoOperand<'a, T, Postgres, OpAst, X> {
    fn into_projection(self) -> Projection<'a, Postgres, X> {
        Projection::new(self.into_operand())
    }
}

impl<'a, T, X> IntoArg<Projection<'a, Postgres, X>> for ArrayAgg<'a, T>
    where
        Self: IntoProjection<'a, Postgres, X> {
    fn into_arg(self) -> Projection<'a, Postgres, X> {
        self.into_projection()
    }
}

impl<'a, T> ArrayAgg<'a, T>
    where
        Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()> {
    pub fn eq<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().eq(v)
        // IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).eq(v)
    }

    pub fn ne<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().ne(v)
    }

    pub fn gt<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().gt(v)
    }

    pub fn ge<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().ge(v)
    }

    pub fn lt<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().lt(v)
    }

    pub fn le<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().le(v)
    }

    pub fn is_null<X>(self) -> Condition<'a, Postgres, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).is_null()
    }

    pub fn is_not_null<X>(self) -> Condition<'a, Postgres, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self)
            .is_not_null()
    }

    pub fn between<L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, Postgres, X>
        where
            L: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X>,
            U: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().between(lower, upper)
    }

    pub fn not_between<L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, Postgres, X>
        where
            L: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X>,
            U: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().not_between(lower, upper)
    }

    pub fn in_list<I, V, K, X>(self, values: I) -> Condition<'a, Postgres, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        self.into_comparator().in_list(values)
    }

    pub fn not_in_list<I, V, Target, K, X>(self, values: I) -> Condition<'a, Postgres, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
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
