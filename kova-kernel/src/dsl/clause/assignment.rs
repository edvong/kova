use crate::dsl::Assignment;
use crate::types::Database;

pub struct AssignmentClause<'a, DB: Database> {
    pub(crate) items: Vec<Assignment<'a, DB, ()>>,
}

impl<'a, DB: Database> AssignmentClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn set(&mut self, assignments: Vec<Assignment<'a, DB, ()>>) {
        self.items = assignments;
    }
}
