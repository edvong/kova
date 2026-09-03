use crate::dsl::{Condition, IntoArgs, Projection, ProjectionClause, WhereClause};
use crate::render::RenderData;
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct DeleteStatement<'a, T, DB: Database> {
    pub(crate) r#where: WhereClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, T, DB: Database> DeleteStatement<'a, T, DB>
where
    T: Model<DB = DB>,
{
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

    pub fn r#where(&mut self, cond: Condition<'a, DB>) {
        self.r#where.set(cond);
    }

    pub fn returning<P>(&mut self, cols: P)
    where
        P: IntoArgs<Projection<'a, DB>>,
    {
        self.returning.set(cols);
    }
}