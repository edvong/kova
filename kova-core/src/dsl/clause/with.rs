use crate::statement::SelectStatement;
use crate::types::Database;

pub struct WithClause<'a, DB: Database> {
    pub(crate) items: Vec<CteNode<'a, DB>>,
}

pub struct CteNode<'a, DB: Database> {
    pub name: &'static str,
    pub statement: Box<SelectStatement<'a, DB>>,
}
