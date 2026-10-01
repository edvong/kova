use crate::dsl::{
    BuildOrdering, BuildProjection, Condition, Direction, IntoComparator, IntoIdentifier,
    IntoOperand, OpAst, OpNode, Ordering, Projection,
};
use crate::types::Database;
use crate::{impl_base_traits_for_std_func, impl_into_func_for_scalar};

pub struct Length<E>(pub E);

impl<E: OpNode> OpNode for Length<E> {
    type Input = String;
    type Output = String;
}

impl_into_func_for_scalar!(Length);
impl_base_traits_for_std_func!(Length);

impl<E> Length<E> {
    // @GENERATED_COMMON_PROJECTION_START
    pub fn alias<'a, DB: Database, A, U, X>(self, alias: A) -> Projection<'a, DB, X>
        where
            Self: OpNode,
            E: OpNode + IntoOperand<'a, <Self as OpNode>::Input, DB, OpAst, X>,
            A: IntoIdentifier<U> {
        self.build_projection(alias)
    }

    pub fn asc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X>
        where
            Self: OpNode,
            E: OpNode + IntoOperand<'a, <Self as OpNode>::Input, DB, OpAst, X> {
        self.build_ordering(Some(Direction::Asc))
    }

    pub fn desc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X>
        where
            Self: OpNode,
            E: OpNode + IntoOperand<'a, <Self as OpNode>::Input, DB, OpAst, X> {
        self.build_ordering(Some(Direction::Desc))
    }
    // @GENERATED_COMMON_PROJECTION_END

    // @GENERATED_COMMON_COMPARATOR_START
    pub fn eq<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).eq(v)
    }

    pub fn ne<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).ne(v)
    }

    pub fn gt<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).gt(v)
    }

    pub fn ge<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).ge(v)
    }

    pub fn lt<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).lt(v)
    }

    pub fn le<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).le(v)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_null<'a, DB: Database, X>(self) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).is_null()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_null<'a, DB: Database, X>(self) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).is_not_null()
    }

    pub fn between<'a, DB: Database, L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            L: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X>,
            U: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self)
            .between(lower, upper)
    }

    pub fn not_between<'a, DB: Database, L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            L: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X>,
            U: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self)
            .not_between(lower, upper)
    }

    pub fn in_list<'a, DB: Database, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).in_list(values)
    }

    pub fn not_in_list<'a, DB: Database, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self)
            .not_in_list(values)
    }

    pub fn in_select<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).in_select(v)
    }

    pub fn not_in_select<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: OpNode + IntoOperand<'a, <Self as OpNode>::Output, DB, OpAst, ()>,
            V: IntoOperand<'a, <Self as OpNode>::Output, DB, K, X> {
        IntoComparator::<'a, <Self as OpNode>::Output, DB>::into_comparator(self).not_in_select(v)
    }
    // @GENERATED_COMMON_COMPARATOR_END
}
