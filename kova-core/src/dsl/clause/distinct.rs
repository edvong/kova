
pub struct DistinctClause {
    distinct: bool,
}

impl DistinctClause {
    pub(crate) fn new() -> Self {
        Self { distinct: false }
    }
    
    pub fn set(&mut self, distinct: bool) {
        self.distinct = distinct;
    }
}