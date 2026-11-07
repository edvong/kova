use crate::Result;
use crate::dsl::{Condition, Projection, ProjectionClause, WhereClause};
use crate::render::RenderData;
use crate::render::{Render, RenderContext, Renderable};
use crate::types::{Database, Relation};
use std::marker::PhantomData;

pub struct DeleteStatement<'a, DB: Database, T> {
    pub(crate) r#where: WhereClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, DB: Database, T: Relation> DeleteStatement<'a, DB, T> {
    pub fn new() -> Self {
        Self {
            r#where: WhereClause::new(),
            returning: ProjectionClause::new(),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn build<'s>(&'s self) -> Result<RenderData<'s, DB>>
        where
            'a: 's,
            DB: Renderable<'a, 's> {
        let mut ctx = RenderContext::new();
        self.render(&mut ctx)?;
        Ok(ctx.into_data())
    }

    pub fn r#where(&mut self, cond: Condition<'a, DB, ()>) {
        self.r#where.set(cond);
    }

    pub fn returning(&mut self, cols: Vec<Projection<'a, DB, ()>>) {
        self.returning.set(cols);
    }
}
