use crate::Result;
use crate::dsl::{Bind, BindClause};
use crate::statement::StatementPayload;
use crate::types::Database;
use std::borrow::Cow;

pub struct RawStatement<'a, DB: Database> {
    pub(crate) sql: Cow<'a, str>,
    pub(crate) binds: BindClause<'a, DB>,
}

impl<'a, DB: Database> RawStatement<'a, DB> {
    pub fn new<S: Into<Cow<'a, str>>>(sql: S) -> Self {
        Self { sql: sql.into(), binds: BindClause::new() }
    }

    pub fn bind(&mut self, args: Vec<Bind<'a, DB>>) {
        self.binds.set(args);
    }

    pub fn build(self) -> Result<StatementPayload<'a, DB>> {
        Ok(StatementPayload { sql: self.sql, params: self.binds.to_values()? })
    }
}
