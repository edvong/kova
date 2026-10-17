use crate::dsl::{
    Assignment, AssignmentClause, Condition, IntoRelationList, Projection, ProjectionClause,
    TableClause, WhereClause,
};
use crate::render::RenderData;
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct UpdateStatement<'a, DB: Database, T> {
    pub(crate) target: &'static str,
    pub(crate) assignments: AssignmentClause<'a, DB>,
    pub(crate) r#where: WhereClause<'a, DB>,
    pub(crate) from: TableClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, DB: Database, T> UpdateStatement<'a, DB, T>
    where
        T: Model<DB = DB> {
    pub fn new() -> Self {
        Self {
            target: T::TABLE_NAME,
            assignments: AssignmentClause::new(),
            r#where: WhereClause::new(),
            from: TableClause::new(),
            returning: ProjectionClause::new(),
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> RenderData<'a, DB> {
        unimplemented!()
    }

    pub fn set(&mut self, assignments: Vec<Assignment<'a, DB, ()>>) {
        self.assignments.set(assignments);
    }

    pub fn from<R: IntoRelationList>(&mut self) {
        for table in R::relation_list() {
            self.from.add_table(table);
        }
    }

    pub fn r#where(&mut self, cond: Condition<'a, DB, ()>) {
        self.r#where.set(cond);
    }

    pub fn returning(&mut self, cols: Vec<Projection<'a, DB, ()>>) {
        self.returning.set(cols);
    }
}
