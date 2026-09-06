mod select;
mod insert;
mod update;
mod delete;
mod raw;

use std::borrow::Cow;
use crate::types::Database;

pub use {
    select::SelectStatement,
    insert::InsertStatement,
    update::UpdateStatement,
    delete::DeleteStatement,
    raw::RawStatement,
};
use crate::render::RenderData;

pub struct StatementPayload<'a, DB: Database> {
    pub sql: Cow<'a, str>,
    pub params: Vec<DB::Value<'a>>,
}

impl<'a, DB: Database> From<RenderData<'a, DB>> for StatementPayload<'a, DB> {
    fn from(data: RenderData<'a, DB>) -> Self {
        Self {
            sql: Cow::Owned(data.sql),
            params: data.params,
        }
    }
}
