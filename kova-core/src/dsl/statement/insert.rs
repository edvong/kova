use crate::dsl::{
    Condition, EraseX, IntoInsertSource, OnConflictClause, Projection, ProjectionClause,
    ValuesClause, WhereClause,
};
use crate::render::RenderData;
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct InsertStatement<'a, DB: Database, T> {
    pub(crate) values: ValuesClause<'a, DB, T>,
    pub(crate) on_conflict: OnConflictClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    pub(crate) r#where: WhereClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, DB: Database, T> InsertStatement<'a, DB, T>
    where
        T: Model<DB = DB> {
    pub fn new<X>(data: impl IntoInsertSource<'a, DB, T, X>) -> Self {
        Self {
            values: ValuesClause::new(T::COLUMNS_META.to_vec(), data.into_source()),
            on_conflict: OnConflictClause::new(),
            returning: ProjectionClause::new(),
            r#where: WhereClause::new(),
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> Vec<RenderData<'a, DB>> {
        unimplemented!()
    }

    pub fn retuning(&mut self, cols: Vec<Projection<'a, DB, ()>>) {
        self.returning.set(cols);
    }

    pub fn conflict_mut(&mut self) -> &mut OnConflictClause<'a, DB> {
        &mut self.on_conflict
    }

    pub fn r#where<X>(&mut self, cond: Condition<'a, DB, X>) {
        self.r#where.set(cond.erase_x())
    }
}
