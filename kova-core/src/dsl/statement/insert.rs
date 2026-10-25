use crate::Result;
use crate::dsl::{
    Condition, IntoInsertSource, OnConflictClause, Projection, ProjectionClause, ValuesClause
};
use crate::render::{RenderData, Renderable};
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct InsertStatement<'a, DB: Database, T, X> {
    pub(crate) target: &'static str,
    pub(crate) values: ValuesClause<'a, DB, T>,
    pub(crate) on_conflict: OnConflictClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    _marker: PhantomData<(T, X)>,
}

impl<'a, DB: Database, T, X> InsertStatement<'a, DB, T, X> {
    pub fn new(data: impl IntoInsertSource<'a, DB, T, X>) -> Self
        where
            T: Model<DB, X> {
        Self {
            target: T::NAME,
            values: ValuesClause::new(T::COLUMNS_META, data.into_source()),
            on_conflict: OnConflictClause::new(),
            returning: ProjectionClause::new(),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn build<'s>(&'s self) -> Result<Vec<RenderData<'s, DB>>>
        where
            'a: 's,
            T: Model<DB, X>,
            DB: Renderable<'a, 's> {
        self.render_batches()
    }

    pub fn returning(&mut self, cols: Vec<Projection<'a, DB, ()>>) {
        self.returning.set(cols);
    }

    pub fn conflict_mut(&mut self) -> &mut OnConflictClause<'a, DB> {
        &mut self.on_conflict
    }

    pub fn r#where(&mut self, cond: Condition<'a, DB, ()>) {
        self.on_conflict.set_where(cond)
    }
}
