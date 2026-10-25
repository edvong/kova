use crate::dsl::{EraseX, Operand};
use crate::types::Database;
use std::marker::PhantomData;

pub struct GroupByClause<'a, DB: Database> {
    pub(crate) items: Vec<GroupBy<'a, DB, ()>>,
}

impl<'a, DB: Database> GroupByClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn set(&mut self, args: Vec<GroupBy<'a, DB, ()>>) {
        self.items = args;
    }
}

pub struct GroupBy<'a, DB: Database, X> {
    pub(crate) operand: Operand<'a, DB>,
    _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> GroupBy<'a, DB, X> {
    pub fn new(operand: Operand<'a, DB>) -> Self {
        Self { operand, _marker: PhantomData }
    }
}

pub trait IntoGroupBy<'a, DB: Database, X> {
    fn into_group_by(self) -> GroupBy<'a, DB, X>;
}

impl<'a, DB: Database, X> EraseX for GroupBy<'a, DB, X> {
    type Erased = GroupBy<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        GroupBy { operand: self.operand, _marker: PhantomData }
    }
}
