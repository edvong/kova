use crate::dsl::Condition;
use crate::types::Database;

pub struct WhereClause<'a, DB: Database> {
    cond: Option<Condition<'a, DB, ()>>,
}

impl<'a, DB: Database> WhereClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { cond: None }
    }

    pub fn from(cond: Condition<'a, DB, ()>) -> Self {
        Self { cond: Some(cond) }
    }

    pub fn set(&mut self, cond: Condition<'a, DB, ()>) {
        self.cond = Some(cond);
    }
}
