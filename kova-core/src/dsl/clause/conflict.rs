use crate::dsl::component::{Component, Std};
use crate::dsl::{Assignment, Condition, EraseX, IntoArg, Operand, WhereClause};
use crate::types::{Database, Extension};
use std::marker::PhantomData;

pub type ExtConflictTarget<DB> = <<DB as Database>::Extension as Extension>::ConflictTarget;

pub struct OnConflictClause<'a, DB: Database> {
    pub target: Option<Component<Vec<Conflict<'a, DB, ()>>, ExtConflictTarget<DB>>>,
    pub action: Option<ConflictAction<'a, DB>>,
}

pub enum ConflictAction<'a, DB: Database> {
    DoNothing,
    DoUpdate {
        assignments: Vec<Assignment<'a, DB, ()>>,
        r#where: WhereClause<'a, DB>,
    },
}

impl<'a, DB: Database> OnConflictClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { target: None, action: None }
    }

    pub fn set_target(
        &mut self, target: Component<Vec<Conflict<'a, DB, ()>>, ExtConflictTarget<DB>>,
    ) {
        self.target = Some(target);
    }

    pub fn set_do_nothing(&mut self) {
        self.action = Some(ConflictAction::DoNothing);
    }

    pub fn set_do_update(&mut self, assignments: Vec<Assignment<'a, DB, ()>>) {
        self.action = Some(ConflictAction::DoUpdate { assignments, r#where: WhereClause::new() });
    }

    pub fn set_where(&mut self, cond: Condition<'a, DB, ()>) {
        if let Some(ConflictAction::DoUpdate { r#where, .. }) = &mut self.action {
            r#where.set(cond);
        }
    }
}

pub struct Conflict<'a, DB: Database, X> {
    pub operand: Operand<'a, DB>,
    _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> Conflict<'a, DB, X> {
    #[inline(always)]
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

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used as an `ON CONFLICT` target",
    label = "invalid `ON CONFLICT` target",
    note = "expected a physical table column, a tuple of columns/scalar expressions, or a platform-specific conflict target."
)]
pub trait IntoConflictTarget<'a, DB: Database, Marker, X> {
    fn into_conflict_target(self) -> Component<Vec<Conflict<'a, DB, ()>>, ExtConflictTarget<DB>>;
}

impl<'a, DB: Database, M, X, T> IntoConflictTarget<'a, DB, (Std, M), X> for T
    where
        T: IntoArg<Conflict<'a, DB, X>, M> {
    #[inline(always)]
    fn into_conflict_target(self) -> Component<Vec<Conflict<'a, DB, ()>>, ExtConflictTarget<DB>> {
        Component::standard(self.into_args_erased())
    }
}
