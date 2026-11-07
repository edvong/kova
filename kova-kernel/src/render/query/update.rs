use crate::Result;
use crate::dsl::statement::UpdateStatement;
use crate::render::{Render, RenderContext, Renderable};

impl<'a: 's, 's, DB: Renderable<'a, 's>, T> Render<'a, 's, DB> for UpdateStatement<'a, DB, T> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        ctx.sql.push_str("update ");
        ctx.push_ident(self.target);
        ctx.sql.push_str(" set ");
        self.assignments.render(ctx)?;
        self.from.render(ctx)?;
        self.r#where.render(ctx)?;

        if !self.returning.items.is_empty() {
            ctx.sql.push_str(" returning ");
            self.returning.render(ctx)?;
        }

        Ok(())
    }
}
