use crate::Result;
use crate::dsl::statement::SelectStatement;
use crate::render::{Render, RenderContext, Renderable};

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for SelectStatement<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        // Render CTEs before entering the main query's tag scope.
        self.with.render(ctx)?;

        let saved_scope = ctx.tags.enter_scope();
        let scope_start = saved_scope;

        // Pre-register tags in strict FROM -> JOIN order.
        for item in &self.from.items {
            if let Some(raw_id) = item.get_tag() {
                ctx.tags.register_in_current_scope(scope_start, raw_id);
            }
        }
        for join in &self.join.items {
            if let Some(raw_id) = join.tag {
                ctx.tags.register_in_current_scope(scope_start, raw_id);
            }
        }

        ctx.sql.push_str("select ");
        self.distinct.render(ctx)?;
        self.projection.render(ctx)?;
        self.from.render(ctx)?;
        self.join.render(ctx)?;
        self.r#where.render(ctx)?;
        self.group_by.render(ctx)?;
        self.having.render(ctx)?;

        if self.combine.items.is_empty() {
            self.order_by.render(ctx)?;
            ctx.tags.exit_scope(saved_scope);
        } else {
            // Exit the left query's tag scope before rendering sibling set-operation queries.
            ctx.tags.exit_scope(saved_scope);
            self.combine.render(ctx)?;

            // Trailing ORDER BY after set operations must use unqualified column identifiers.
            let prev_unqualify = ctx.unqualify_columns;
            ctx.unqualify_columns = true;
            self.order_by.render(ctx)?;
            ctx.unqualify_columns = prev_unqualify;
        }

        self.pagination.render(ctx)?;
        self.locking.render(ctx)?;

        Ok(())
    }
}
