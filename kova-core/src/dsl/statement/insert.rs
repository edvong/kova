use crate::dsl::{
    EraseX, ExtOnConflict, ExtensionClause, InsertRows, IntoArgs, OnConflictClause, Projection,
    ProjectionClause, ValuesClause,
};
use crate::render::RenderData;
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct InsertStatement<'a, DB: Database, T> {
    pub(crate) values: ValuesClause<'a, T>,
    pub(crate) on_conflict: OnConflictClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, DB: Database, T> InsertStatement<'a, DB, T>
    where
        T: Model<DB = DB> {
    pub fn new(rows: InsertRows<'a, T>) -> Self {
        Self {
            values: ValuesClause::new(T::COLUMNS_META.to_vec(), rows),
            on_conflict: OnConflictClause::new(),
            returning: ProjectionClause::new(),
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> Vec<RenderData<'a, DB>> {
        unimplemented!()
    }

    pub fn retuning<A, X>(&mut self, cols: A)
        where
            A: IntoArgs<Projection<'a, DB, X>> {
        self.returning.set(cols.into_mapped_args(|p| p.erase_x()));
    }

    pub fn on_conflict_ext_mut_or<F>(&mut self, f: F) -> &mut ExtOnConflict<'a, DB>
        where
            F: FnOnce() -> ExtOnConflict<'a, DB> {
        self.on_conflict.ext_mut_or_with(f)
    }
}
