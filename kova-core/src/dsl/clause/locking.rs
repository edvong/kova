use crate::dsl::ExtensionClause;
use crate::types::{Database, Extension};

pub type ExtLocking<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Locking;

pub struct LockingClause<'a, DB: Database> {
    pub(crate) locking: Option<ExtLocking<'a, DB>>,
}

impl<'a, DB: Database> LockingClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { locking: None }
    }
}

impl<'a, DB: Database> ExtensionClause<ExtLocking<'a, DB>> for LockingClause<'a, DB> {
    fn ext_opt(&self) -> &Option<ExtLocking<'a, DB>> {
        &self.locking
    }

    fn ext_opt_mut(&mut self) -> &mut Option<ExtLocking<'a, DB>> {
        &mut self.locking
    }
}
