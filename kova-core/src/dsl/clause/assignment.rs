use crate::dsl::{Assignment, IntoArgs};
use crate::types::Database;

pub struct AssignmentClause<'a, DB: Database> {
    pub(crate) items: Vec<Assignment<'a, DB>>,
}

impl<'a, DB: Database> AssignmentClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn set<A>(&mut self, args: A)
    where
        A: IntoArgs<Assignment<'a, DB>>,
    {
        self.items.clear();
        args.append_args(&mut self.items);
    }
}