use crate::dsl::ExtensionClause;
use crate::types::{Database, Extension};

pub type ExtOnConflict<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::OnConflict<'a>;

pub struct OnConflictClause<'a, DB: Database> {
    inner: ExtOnConflict<'a, DB>,
}

impl<'a, DB: Database> OnConflictClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { inner: Default::default() }
    }
}

impl<'a, DB: Database> ExtensionClause<ExtOnConflict<'a, DB>> for OnConflictClause<'a, DB> {
    fn ext(&self) -> &ExtOnConflict<'a, DB> {
        &self.inner
    }

    fn ext_mut(&mut self) -> &mut ExtOnConflict<'a, DB> {
        &mut self.inner
    }
}
