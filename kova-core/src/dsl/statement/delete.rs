use crate::dsl::{Condition, Projection, ProjectionClause, WhereClause};
use crate::render::RenderData;
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct DeleteStatement<'a, DB: Database, T> {
    pub(crate) r#where: WhereClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, DB: Database, T> DeleteStatement<'a, DB, T> where T: Model<DB = DB> {
    pub fn new() -> Self {
        Self {
            r#where: WhereClause::new(),
            returning: ProjectionClause::new(),
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> RenderData<'a, DB> {
        unimplemented!()
    }

    pub fn r#where(&mut self, cond: Condition<'a, DB, ()>) {
        self.r#where.set(cond);
    }

    pub fn returning(&mut self, cols: Vec<Projection<'a, DB, ()>>) {
        self.returning.set(cols);
    }
}
