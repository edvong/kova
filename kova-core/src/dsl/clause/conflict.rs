use crate::dsl::{Assignment, Operand};
use crate::types::Database;

pub struct OnConflictClause<'a, DB: Database> {
    pub(crate) targets: Vec<Operand<'a, DB>>,
    pub(crate) action: Option<ConflictAction<'a, DB>>,
}

pub enum ConflictAction<'a, DB: Database> {
    DoNothing,
    DoUpdate(Vec<Assignment<'a, DB, ()>>),
}

impl<'a, DB: Database> OnConflictClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { targets: Vec::new(), action: None }
    }

    pub fn set_targets(&mut self, targets: Vec<Operand<'a, DB>>) {
        self.targets = targets;
    }

    pub fn set_do_nothing(&mut self) {
        self.action = Some(ConflictAction::DoNothing);
    }

    pub fn set_do_update(&mut self, assignments: Vec<Assignment<'a, DB, ()>>) {
        self.action = Some(ConflictAction::DoUpdate(assignments));
    }
}
