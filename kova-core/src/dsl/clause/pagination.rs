use crate::dsl::ExtensionClause;
use crate::types::{Database, Extension};

pub type ExtPagination<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Pagination;

pub struct PaginationClause<'a, DB: Database> {
    pagination: Option<ExtPagination<'a, DB>>,
}

impl<'a, DB: Database> PaginationClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { pagination: None }
    }
}

impl<'a, DB: Database> ExtensionClause<ExtPagination<'a, DB>> for PaginationClause<'a, DB> {
    fn ext_opt(&self) -> &Option<ExtPagination<'a, DB>> {
        &self.pagination
    }

    fn ext_opt_mut(&mut self) -> &mut Option<ExtPagination<'a, DB>> {
        &mut self.pagination
    }
}
