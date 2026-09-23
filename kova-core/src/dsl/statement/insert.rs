use crate::dsl::{
    Condition, EraseX, ExtOnConflict, ExtensionClause, InsertRows, IntoArgs, OnConflictClause,
    Projection, ProjectionClause, ValuesClause, WhereClause,
};
use crate::render::RenderData;
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct InsertStatement<'a, DB: Database, T> {
    pub(crate) values: ValuesClause<'a, T>,
    pub(crate) on_conflict: OnConflictClause<'a, DB>,
    pub(crate) returning: ProjectionClause<'a, DB>,
    pub(crate) r#where: WhereClause<'a, DB>,
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
            r#where: WhereClause::new(),
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

    pub fn conflict_mut(&mut self) -> &mut ExtOnConflict<'a, DB> {
        self.on_conflict.ext_mut()
    }

    pub fn r#where<X>(&mut self, cond: Condition<'a, DB, X>) {
        self.r#where.set(cond.erase_x())
    }
}
