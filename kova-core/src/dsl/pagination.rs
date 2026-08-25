use crate::dsl::{ExtensionClause, Component};
use crate::types::{Database, Extension};

pub type ExtPagination<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Pagination;

pub struct PaginationClause<'a, DB: Database> {
    pagination: Option<Component<(), ExtPagination<'a, DB>>>,
}

impl<'a, DB: Database> PaginationClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { pagination: None }
    }
}

impl<'a, DB: Database> ExtensionClause for PaginationClause<'a, DB> {
    type S = ();
    type E = ExtPagination<'a, DB>;

    fn component(&self) -> &Option<Component<Self::S, Self::E>> {
        &self.pagination
    }

    fn component_mut(&mut self) -> &mut Option<Component<Self::S, Self::E>> {
        &mut self.pagination
    }
}
