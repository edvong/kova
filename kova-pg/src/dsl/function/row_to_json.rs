use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::{
    Condition, Direction, IntoComparator, IntoIdentifier, IntoOperand, IntoOrdering,
    IntoProjection, OpAst, OpNode, Ordering, Projection,
};
use kova_core::{impl_base_traits_for_func, impl_into_func_for_scalar};

pub struct RowToJson<E>(pub E);

impl<E: OpNode> OpNode for RowToJson<E> {
    type Input = E::Output;
    type Output = String;
}

impl_into_func_for_scalar!(RowToJson, Postgres, ExtFunction::RowToJson);
impl_base_traits_for_func!(RowToJson, Postgres);

impl<E> RowToJson<E> {
    // @GENERATED_COMMON_PROJECTION_START
    pub fn alias<'a, A, U, X>(self, alias: A) -> Projection<'a, Postgres, X>
        where
            Self: OpNode,
            E: OpNode + IntoOperand<'a, <Self as OpNode>::Input, Postgres, OpAst, X>,
            A: IntoIdentifier<U> {
        self.into_projection().alias(alias)
    }

    pub fn asc<'a, X>(self) -> Ordering<'a, Postgres, X>
        where
            Self: OpNode,
            E: OpNode + IntoOperand<'a, <Self as OpNode>::Input, Postgres, OpAst, X> {
        self.into_ordering().direction(Direction::Asc)
    }

    pub fn desc<'a, X>(self) -> Ordering<'a, Postgres, X>
        where
            Self: OpNode,
            E: OpNode + IntoOperand<'a, <Self as OpNode>::Input, Postgres, OpAst, X> {
        self.into_ordering().direction(Direction::Desc)
    }
    // @GENERATED_COMMON_PROJECTION_END

    // @GENERATED_COMMON_COMPARATOR_START
    pub fn eq<'a, V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).eq(v)
    }

    pub fn ne<'a, V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).ne(v)
    }

    pub fn gt<'a, V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).gt(v)
    }

    pub fn ge<'a, V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).ge(v)
    }

    pub fn lt<'a, V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).lt(v)
    }

    pub fn le<'a, V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).le(v)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_null<'a, X>(self) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).is_null()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_null<'a, X>(self) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).is_not_null()
    }

    pub fn between<'a, L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            L: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X>,
            U: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self)
            .between(lower, upper)
    }

    pub fn not_between<'a, L, U, K, X>(
        self, lower: L, upper: U,
    ) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            L: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X>,
            U: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self)
            .not_between(lower, upper)
    }

    pub fn in_list<'a, I, V, K, X>(self, values: I) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).in_list(values)
    }

    pub fn not_in_list<'a, I, V, K, X>(self, values: I) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self)
            .not_in_list(values)
    }

    pub fn in_select<'a, V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).in_select(v)
    }

    pub fn not_in_select<'a, V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, Postgres, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, Postgres, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, Postgres>::into_comparator(self).not_in_select(v)
    }
    // @GENERATED_COMMON_COMPARATOR_END
}
