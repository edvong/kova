use crate::Result;
use crate::dsl::{Bind, BindClause, IntoArgs};
use crate::statement::RawRenderData;
use crate::types::Database;
use std::borrow::Cow;

pub struct RawStatement<'a, DB: Database> {
    pub(crate) sql: Cow<'a, str>,
    pub(crate) binds: BindClause<'a, DB>,
}

impl<'a, DB: Database> RawStatement<'a, DB> {
    pub fn new<S: Into<Cow<'a, str>>>(sql: S) -> Self {
        Self {
            sql: sql.into(),
            binds: BindClause::new(),
        }
    }

    pub fn bind<A>(&mut self, args: A)
    where
        A: IntoArgs<Bind<'a, DB>>,
    {
        self.binds.bind(args);
    }

    pub fn build(self) -> Result<RawRenderData<'a, DB>> {
        Ok(RawRenderData {
            sql: self.sql,
            params: self.binds.to_values()?,
        })
    }
}

