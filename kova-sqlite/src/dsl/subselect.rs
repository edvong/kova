use crate::types::Sqlite;
use kova_core::dsl::statement::{IntoSelectStatement, SelectStatement};
use kova_core::dsl::{
    Condition, GroupBy, InsertSource, IntoArgs, IntoIdentifier, IntoInsertSource,
    IntoJoinConstraint, IntoOperand, JoinType, OpAst, Operand, Ordering, Projection,
    ProjectionExpr, SetOperator, SingleArgMarker,
};
use kova_core::types::{Model, Relation};
use std::marker::PhantomData;

pub struct Entry;
pub struct Chain<X>(PhantomData<X>);

pub struct Subselect<'a, T: Model, State = Entry> {
    pub(crate) statement: SelectStatement<'a, T::DB>,
    pub(crate) _marker: PhantomData<(T, State)>,
}

impl<'a, T: Model<DB = Sqlite>> Subselect<'a, T, Entry> {
    pub fn columns<A, X>(cols: A) -> Subselect<'a, T, Chain<X>>
        where
            A: IntoArgs<Projection<'a, Sqlite, X>> {
        let mut statement = SelectStatement::new(T::TABLE_NAME);
        statement.columns(cols);
        Subselect { statement, _marker: PhantomData }
    }
}

impl<'a, T: Model<DB = Sqlite>, X> Subselect<'a, T, Chain<X>> {
    pub fn inner_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, T::DB, X>,
    ) -> Self {
        self.statement.join::<R, X>(JoinType::Inner, constraint);
        self
    }

    pub fn left_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, T::DB, X>,
    ) -> Self {
        self.statement.join::<R, X>(JoinType::Left, constraint);
        self
    }

    pub fn right_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, T::DB, X>,
    ) -> Self {
        self.statement.join::<R, X>(JoinType::Right, constraint);
        self
    }

    pub fn full_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, T::DB, X>,
    ) -> Self {
        self.statement.join::<R, X>(JoinType::Full, constraint);
        self
    }

    pub fn cross_join<R: Relation>(mut self) -> Self {
        self.statement.cross_join::<R>();
        self
    }

    pub fn filter(mut self, cond: Condition<'a, T::DB, X>) -> Self {
        self.statement.r#where(cond);
        self
    }

    pub fn group_by<A: IntoArgs<GroupBy<'a, T::DB, X>>>(mut self, cols: A) -> Self {
        self.statement.group_by(cols);
        self
    }

    pub fn having(mut self, cond: Condition<'a, T::DB, X>) -> Self {
        self.statement.having(cond);
        self
    }

    pub fn order_by<A: IntoArgs<Ordering<'a, T::DB, X>>>(mut self, cols: A) -> Self {
        self.statement.order_by(cols);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.statement.limit(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.statement.offset(offset);
        self
    }

    pub fn union(mut self, other: impl IntoSelectStatement<'a, T::DB, X>) -> Self {
        self.statement.combine(SetOperator::Union, other.into_select_statement());
        self
    }

    pub fn union_all(mut self, other: impl IntoSelectStatement<'a, T::DB, X>) -> Self {
        self.statement.combine(SetOperator::UnionAll, other.into_select_statement());
        self
    }

    pub fn intersect(mut self, other: impl IntoSelectStatement<'a, T::DB, X>) -> Self {
        self.statement.combine(SetOperator::Intersect, other.into_select_statement());
        self
    }

    pub fn except(mut self, other: impl IntoSelectStatement<'a, T::DB, X>) -> Self {
        self.statement.combine(SetOperator::Except, other.into_select_statement());
        self
    }

    pub fn alias<I: IntoIdentifier<A>, A>(self, alias: I) -> Projection<'a, T::DB, X> {
        Projection::new(
            ProjectionExpr::Subselect(Box::new(self.statement)),
            Some(alias.into_identifier().meta),
        )
    }
}

impl<'a, T: Model, X> SingleArgMarker for Subselect<'a, T, Chain<X>> {}

impl<'a, T: Model<DB = Sqlite>, X> IntoOperand<'a, T, Sqlite, OpAst, X>
    for Subselect<'a, T, Chain<X>> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_subselect(self.statement)
    }
}

impl<'a, T: Model<DB = Sqlite>, X> IntoSelectStatement<'a, Sqlite, X>
    for Subselect<'a, T, Chain<X>> {
    #[inline(always)]
    fn into_select_statement(self) -> SelectStatement<'a, Sqlite> {
        self.statement
    }
}

impl<'a, T, M, X> IntoInsertSource<'a, T::DB, T, X> for Subselect<'a, M, Chain<X>>
    where
        T: Model<DB = Sqlite>,
        M: Model<DB = Sqlite> {
    #[inline(always)]
    fn into_source(self) -> InsertSource<'a, Sqlite, T> {
        IntoInsertSource::<'a, T::DB, T, X>::into_source(self.into_select_statement())
    }
}
