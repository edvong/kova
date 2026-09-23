pub struct FromClause {
    items: Vec<&'static str>,
}

impl FromClause {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn set(&mut self, items: Vec<&'static str>) {
        self.items = items;
    }
}
