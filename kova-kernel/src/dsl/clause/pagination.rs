pub struct PaginationClause {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl PaginationClause {
    pub(crate) fn new() -> Self {
        Self { limit: None, offset: None }
    }

    pub fn limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    pub fn offset(&mut self, offset: usize) {
        self.offset = Some(offset);
    }
}
