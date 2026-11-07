use crate::dsl::statement::SelectStatement;
use crate::types::Database;

#[derive(Debug, Clone, Copy)]
pub enum SetOperator {
    Union,
    UnionAll,
    Intersect,
    Except,
}

pub struct SetOperation<'a, DB: Database> {
    pub operator: SetOperator,
    pub statement: Box<SelectStatement<'a, DB>>,
}

pub struct CombineClause<'a, DB: Database> {
    pub(crate) items: Vec<SetOperation<'a, DB>>,
}

impl<'a, DB: Database> CombineClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, operator: SetOperator, statement: SelectStatement<'a, DB>) {
        self.items.push(SetOperation { operator, statement: Box::new(statement) });
    }
}
