use crate::dsl::Condition;
use crate::types::Database;

pub struct JoinClause<'a, DB: Database> {
    pub(crate) items: Vec<Join<'a, DB>>,
}

impl<'a, DB: Database> JoinClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, join: Join<'a, DB>) {
        self.items.push(join);
    }
}

pub struct Join<'a, DB: Database> {
    pub typ: JoinType,
    pub target_table: &'static str,
    pub on: Condition<'a, DB, ()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Inner,
    Left,
}