use crate::dsl::{ExtOnConflict, ExtensionClause, InsertRows, IntoArgs, OnConflictClause, Projection, ProjectionClause, ValuesClause};
use crate::types::{Database, Model};
use std::marker::PhantomData;
use crate::render::RenderData;

pub struct InsertStatement<'a, T, DB: Database> {
    pub(crate) values: ValuesClause<'a, T>,
    pub(crate) on_conflict: OnConflictClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, T, DB: Database> InsertStatement<'a, T, DB>
where
    T: Model<DB = DB>,
{
    pub fn new(rows: InsertRows<'a, T>) -> Self {
        Self {
            values: ValuesClause::new(T::COLUMNS_META.to_vec(), rows),
            on_conflict: OnConflictClause::new(),
            returning: ProjectionClause::new(),
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> RenderData<'a, DB> {
        todo!()
    }


    pub fn retuning<C>(&mut self, cols: C)
    where
        C: IntoArgs<Projection<'a, DB>>,
    {
        self.returning.set(cols);
    }

    pub fn on_conflict_ext_mut_or<F>(&mut self, f: F) -> &mut ExtOnConflict<'a, DB>
    where
        F: FnOnce() -> ExtOnConflict<'a, DB>
    {
        self.on_conflict.ext_mut_or_with(f)
    }




}
