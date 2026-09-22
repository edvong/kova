use crate::dsl::{
    Assignment, AssignmentClause, Condition, EraseX, FromClause, IntoArgs, IntoRelationList,
    Projection, ProjectionClause, WhereClause,
};
use crate::render::RenderData;
use crate::types::{Database, Model, Relation};
use std::marker::PhantomData;

pub struct UpdateStatement<'a, DB: Database, T> {
    pub(crate) assignments: AssignmentClause<'a, DB>,
    pub(crate) r#where: WhereClause<'a, DB>,
    pub(crate) from: FromClause,
    pub(crate) returning: ProjectionClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, DB: Database, T> UpdateStatement<'a, DB, T>
    where
        T: Model<DB = DB> {
    pub fn new() -> Self {
        Self {
            assignments: AssignmentClause::new(),
            r#where: WhereClause::new(),
            from: FromClause::new(),
            returning: ProjectionClause::new(),
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> RenderData<'a, DB> {
        unimplemented!()
    }

    pub fn set<A, X>(&mut self, assignments: A)
        where
            A: IntoArgs<Assignment<'a, DB, X>> {
        self.assignments.set(assignments.into_mapped_args(|a| a.erase_x()));
    }

    pub fn from<R: IntoRelationList>(&mut self) {
        self.from.set(R::relation_list());
    }

    pub fn r#where<X>(&mut self, cond: Condition<'a, DB, X>) {
        self.r#where.set(cond.erase_x());
    }

    pub fn returning<A, X>(&mut self, cols: A)
        where
            A: IntoArgs<Projection<'a, DB, X>> {
        self.returning.set(cols.into_mapped_args(|p| p.erase_x()));
    }
}
