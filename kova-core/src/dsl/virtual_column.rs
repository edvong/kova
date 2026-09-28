use crate::dsl::{
    BuildOrdering, Comparator, Condition, Direction, GroupBy, IntoArg, IntoComparator,
    IntoIdentifier, IntoOperand, OpAst, Operand, OrderOperand, Ordering, Projection,
    ProjectionExpr, SingleArgMarker, StandardGroupBy, StandardOperand, StandardOperator,
    WithComponent,
};
use crate::types::{Database, IdentifierMeta};

// For SRF virtual tables, CTE fields, and other columns without specific physical type constraints.
#[derive(Copy, Clone)]
pub struct VirtualColumn {
    pub meta: IdentifierMeta,
}

impl VirtualColumn {
    pub const fn new(source: &'static str, name: &'static str) -> Self {
        Self { meta: IdentifierMeta::new(source, name) }
    }

    pub fn tag<Marker: 'static>(mut self) -> Self {
        self.meta = self.meta.with_tag(crate::type_id::<Marker>());
        self
    }

    pub fn alias<'a, DB: Database, I: IntoIdentifier<A>, A, X>(
        self, alias: I,
    ) -> Projection<'a, DB, X> {
        Projection::new(ProjectionExpr::Column(self.meta), Some(alias.into_identifier().meta))
    }

    pub fn asc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Asc))
    }

    pub fn desc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Desc))
    }

    pub fn eq<'a, DB: Database, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).binary_op(v, StandardOperator::Eq)
    }

    pub fn ne<'a, DB: Database, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).binary_op(v, StandardOperator::Ne)
    }

    pub fn gt<'a, DB: Database, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).binary_op(v, StandardOperator::Gt)
    }

    pub fn ge<'a, DB: Database, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).binary_op(v, StandardOperator::Ge)
    }

    pub fn lt<'a, DB: Database, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).binary_op(v, StandardOperator::Lt)
    }

    pub fn le<'a, DB: Database, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).binary_op(v, StandardOperator::Le)
    }

    pub fn is_null<'a, DB: Database, X>(self) -> Condition<'a, DB, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).is_null()
    }

    pub fn is_not_null<'a, DB: Database, X>(self) -> Condition<'a, DB, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).is_not_null()
    }

    pub fn between<'a, DB: Database, Target, L, U, K, X>(
        &self, lower: L, upper: U,
    ) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, Target, DB, K, X>,
            U: IntoOperand<'a, Target, DB, K, X> {
        self.into_comparator().ternary_op(lower, upper, StandardOperator::Between)
    }

    pub fn not_between<'a, DB: Database, Target, L, U, K, X>(
        &self, lower: L, upper: U,
    ) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, Target, DB, K, X>,
            U: IntoOperand<'a, Target, DB, K, X> {
        self.into_comparator().ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    pub fn in_list<'a, DB: Database, Target, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self)
            .list_op(values, StandardOperator::InList)
    }

    pub fn not_in_list<'a, DB: Database, Target, I, V, K, X>(
        self, values: I,
    ) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self)
            .list_op(values, StandardOperator::NotInList)
    }

    // No Inner Query

    pub fn like<'a, DB: Database, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).binary_op(v, StandardOperator::Like)
    }

    pub fn not_like<'a, DB: Database, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, Target, DB, K, X> {
        IntoComparator::<'a, (), DB>::into_comparator(self).binary_op(v, StandardOperator::NotLike)
    }
}


impl SingleArgMarker for VirtualColumn {}

impl<'a, Target, DB: Database, X> IntoOperand<'a, Target, DB, OpAst, X> for VirtualColumn {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_identifier(self.meta)
    }
}

impl<'a, DB: Database> IntoComparator<'a, (), DB> for VirtualColumn {
    #[inline(always)]
    fn into_comparator(self) -> Comparator<'a, (), DB> {
        Comparator::new(Operand::from_identifier(self.meta))
    }
}

impl<'a, DB: Database, X> IntoArg<Projection<'a, DB, X>> for VirtualColumn {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, DB, X> {
        Projection::new(ProjectionExpr::Column(self.meta), None)
    }
}

impl<'a, DB: Database, X> BuildOrdering<'a, DB, X> for VirtualColumn {
    #[inline(always)]
    fn build_ordering(self, direction: Option<Direction>) -> Ordering<'a, DB, X> {
        Ordering::new(OrderOperand::from_std(StandardOperand::Column(self.meta)), direction, None)
    }
}

impl<'a, DB: Database, X> IntoArg<GroupBy<'a, DB, X>> for VirtualColumn {
    #[inline(always)]
    fn into_arg(self) -> GroupBy<'a, DB, X> {
        GroupBy::standard(StandardGroupBy::Column(self.meta))
    }
}
