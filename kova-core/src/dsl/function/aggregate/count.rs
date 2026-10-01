use crate::dsl::function::{Function, Quantifier, StandardFunction, Window};
use crate::dsl::{BuildOrdering, BuildProjection, Condition, Direction, GroupBy, IntoArg, IntoComparator, IntoIdentifier, IntoOperand, OpAst, OpNode, OrderOperand, Ordering, Projection, ProjectionExpr, StandardGroupBy, StandardOperand, WithComponent};
use crate::types::Database;
use crate::{impl_base_traits_for_std_func, impl_into_func_for_agg};
use std::marker::PhantomData;

pub struct Count<E>(pub E);
pub struct Star;

impl<E: OpNode> OpNode for Count<E> {
    type Input = E::Output;
    type Output = i64;
}

impl OpNode for Count<Star> {
    type Input = ();
    type Output = i64;
}

impl_into_func_for_agg!(Count);
impl_base_traits_for_std_func!(Count);

impl<E: OpNode> Count<E> {
    // @GENERATED_AGGREGATE_OPS_START
    pub fn distinct<'a, DB: Database, X>(self) -> Function<'a, DB>
        where
            Self: OpNode,
            E: OpNode + IntoOperand<'a, <Self as OpNode>::Input, DB, OpAst, X> {
        self.into_func_with(Quantifier::Distinct)
    }

    pub fn over<'a, DB: Database, X>(self) -> Window<'a, DB, X>
        where
            Self: OpNode,
            E: OpNode + IntoOperand<'a, <Self as OpNode>::Input, DB, OpAst, X> {
        Window::new(self.into_func())
    }
    // @GENERATED_AGGREGATE_OPS_END

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
}

impl<E> Count<E> {
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

// Count<Star> Implement

impl<'a, DB: Database, X> BuildProjection<'a, DB, X> for Count<Star> {
    #[inline(always)]
    fn build_projection<A, U>(self, alias: A) -> Projection<'a, DB, X>
        where
            A: IntoIdentifier<U> {
        let func = Function::from_std(StandardFunction::CountAll);
        Projection::new(ProjectionExpr::Function(func), Some(alias.into_identifier().meta))
    }
}

impl<'a, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Count<Star> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, DB, X> {
        let func = Function::from_std(StandardFunction::CountAll);
        Projection::new(ProjectionExpr::Function(func), None)
    }
}

impl<'a, DB: Database, X> BuildOrdering<'a, DB, X> for Count<Star> {
    #[inline(always)]
    fn build_ordering(self, direction: Option<Direction>) -> Ordering<'a, DB, X> {
        let func = Function::from_std(StandardFunction::CountAll);
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Function(func)),
            direction,
            nulls: None,
            _marker: PhantomData,
        }
    }
}

impl<'a, DB: Database, X> IntoArg<GroupBy<'a, DB, X>> for Count<Star> {
    #[inline(always)]
    fn into_arg(self) -> GroupBy<'a, DB, X> {
        let func = Function::from_std(StandardFunction::CountAll);
        GroupBy::standard(StandardGroupBy::Function(func))
    }
}


impl Count<Star> {
    pub fn alias<'a, DB: Database, A, U, X>(self, alias: A) -> Projection<'a, DB, X>
        where
            A: IntoIdentifier<U> {
        self.build_projection(alias)
    }

    pub fn asc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Asc))
    }

    pub fn desc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Desc))
    }

    pub fn over<'a, DB: Database, X>(self) -> Window<'a, DB, X> {
        let func = Function::from_std(StandardFunction::CountAll);
        Window::new(func)
    }
}
