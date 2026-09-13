use crate::render::{Render, RenderData};
use crate::statement::SelectStatement;
use crate::types::Database;

pub struct SelectContext<'ctx, 'a, DB: Database> {
    pub data: &'ctx mut RenderData<'a, DB>,
}

impl<'ctx, 'a, DB: Database> SelectContext<'ctx, 'a, DB> {
    pub fn new(data: &'ctx mut RenderData<'a, DB>) -> Self {
        Self { data }
    }
}


impl<'ctc, 'a, DB> Render<SelectContext<'ctc, 'a, DB>> for SelectStatement<'a, DB>
where DB: Database
{
    fn render(&self, ctx: &mut SelectContext<'ctc, 'a, DB>) {
        todo!()
    }
}