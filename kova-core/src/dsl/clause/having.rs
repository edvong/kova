use crate::dsl::Condition;
use crate::types::Database;

pub struct HavingClause<'a, DB: Database> {
    cond: Option<Condition<'a, DB, ()>>,
}

impl<'a, DB: Database> HavingClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { cond: None }
    }

    pub fn set(&mut self, cond: Condition<'a, DB, ()>) {
        self.cond = Some(cond);
    }
}
