mod delete;
mod insert;
mod raw;
mod select;
mod update;

use crate::types::Database;
use std::borrow::Cow;

use crate::render::RenderData;
pub use {
    delete::DeleteStatement, insert::InsertStatement, raw::RawStatement, select::SelectStatement,
    update::UpdateStatement,
};

pub struct StatementPayload<'a, DB: Database> {
    pub sql: Cow<'a, str>,
    pub params: Vec<DB::Value<'a>>,
}

impl<'a, DB: Database> From<RenderData<'a, DB>> for StatementPayload<'a, DB> {
    fn from(data: RenderData<'a, DB>) -> Self {
        Self { sql: Cow::Owned(data.sql), params: data.params }
    }
}
