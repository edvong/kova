use crate::Result;
use crate::dsl::statement::DeleteStatement;
use crate::render::{Render, RenderContext, Renderable};
use crate::types::Relation;

impl<'a: 's, 's, DB: Renderable<'a, 's>, T: Relation> Render<'a, 's, DB>
    for DeleteStatement<'a, DB, T> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        ctx.sql.push_str("delete from ");
        ctx.push_ident(T::NAME);
        self.r#where.render(ctx)?;

        if !self.returning.items.is_empty() {
            ctx.sql.push_str(" returning ");
            self.returning.render(ctx)?;
        }

        Ok(())
    }
}
