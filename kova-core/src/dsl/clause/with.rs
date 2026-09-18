use crate::dsl::statement::SelectStatement;
use crate::types::Database;

pub struct WithClause<'a, DB: Database> {
    pub(crate) items: Vec<CteNode<'a, DB>>,
}

pub struct CteNode<'a, DB: Database> {
    pub name: &'static str,
    pub statement: Box<SelectStatement<'a, DB>>,
}

impl<'a, DB: Database> WithClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn add(&mut self, node: CteNode<'a, DB>) {
        self.items.push(node);
    }
}
