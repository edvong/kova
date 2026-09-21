use crate::dsl::ExtensionClause;
use crate::types::{Database, Extension};

pub type ExtLocking<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Locking;

pub struct LockingClause<'a, DB: Database> {
    pub(crate) inner: ExtLocking<'a, DB>,
}

impl<'a, DB: Database> LockingClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { inner: Default::default() }
    }
}

impl<'a, DB: Database> ExtensionClause<ExtLocking<'a, DB>> for LockingClause<'a, DB> {
    fn ext(&self) -> &ExtLocking<'a, DB> {
        &self.inner
    }

    fn ext_mut(&mut self) -> &mut ExtLocking<'a, DB> {
        &mut self.inner
    }
}
