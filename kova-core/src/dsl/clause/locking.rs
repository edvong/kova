use crate::dsl::ExtensionClause;
use crate::types::{Database, Extension};

pub type ExtLocking<DB> = <<DB as Database>::Extension as Extension>::Locking;

pub struct LockingClause<DB: Database> {
    pub(crate) inner: ExtLocking<DB>,
}

impl<'a, DB: Database> LockingClause<DB> {
    pub(crate) fn new() -> Self {
        Self { inner: Default::default() }
    }
}

impl<'a, DB: Database> ExtensionClause<ExtLocking<DB>> for LockingClause<DB> {
    fn ext(&self) -> &ExtLocking<DB> {
        &self.inner
    }

    fn ext_mut(&mut self) -> &mut ExtLocking<DB> {
        &mut self.inner
    }
}
