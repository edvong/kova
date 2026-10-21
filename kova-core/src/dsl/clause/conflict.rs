use crate::dsl::{Assignment, EraseX, Operand};
use crate::types::Database;
use std::marker::PhantomData;

pub struct OnConflictClause<'a, DB: Database> {
    pub(crate) targets: Vec<Conflict<'a, DB, ()>>,
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

    pub fn set_targets(&mut self, targets: Vec<Conflict<'a, DB, ()>>) {
        self.targets = targets;
    }

    pub fn set_do_nothing(&mut self) {
        self.action = Some(ConflictAction::DoNothing);
    }

    pub fn set_do_update(&mut self, assignments: Vec<Assignment<'a, DB, ()>>) {
        self.action = Some(ConflictAction::DoUpdate(assignments));
    }
}

pub struct Conflict<'a, DB: Database, X> {
    operand: Operand<'a, DB>,
    _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> Conflict<'a, DB, X> {
    pub fn new(operand: Operand<'a, DB>) -> Self {
        Self { operand, _marker: PhantomData }
    }
}

pub trait IntoConflict<'a, DB: Database, X> {
    fn into_conflict(self) -> Conflict<'a, DB, X>;
}

impl<'a, DB: Database, X> EraseX for Conflict<'a, DB, X> {
    type Erased = Conflict<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Conflict { operand: self.operand, _marker: PhantomData }
    }
}
