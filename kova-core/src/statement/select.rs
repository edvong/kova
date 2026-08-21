use crate::dsl::{Condition, Pagination};
use crate::render::{Render, RenderData, SelectContext};
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct SelectStatement<'a, T, DB: Database> {
    // pub modifier: Option<SelectModifier<'a, DB>>,
    // pub projections: Vec<Expression<'a, DB>>,
    pub where_clause: Option<Condition<'a, DB>>,
    // pub table_hints: Option<TableHint<'a, DB>>,
    // pub lock_clause: Option<LockClause<'a, DB>>,
    pub pagination: Option<Pagination<'a, DB>>,
    _marker: PhantomData<T>,
}

impl<'a, T, DB> SelectStatement<'a, T, DB>
where
    T: Model<DB = DB>,
    DB: Database
{
    pub fn new() -> Self {
        Self {
            where_clause: None,
            pagination: None,
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> RenderData<'a, DB> {
        let mut data = RenderData::new();
        let mut ctx = SelectContext::new(&mut data);
        self.render(&mut ctx);
        data
    }

    // ==== BEGIN: Standard Implementation ====

    pub fn filter(&mut self, cond: Condition<'a, DB>) {
        self.where_clause = Some(cond);
    }

    // More Implementations...

    // ==== END: Standard Implementation ====
}









