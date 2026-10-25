use crate::types::Postgres;
use kova_core::Result;
use kova_core::dsl::PaginationClause;
use kova_core::render::{PlatformRender, RenderContext};

impl<'a, 's> PlatformRender<'a, 's, &'s PaginationClause> for Postgres {
    #[inline(always)]
    fn render(node: &'s PaginationClause, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()> {
        if let Some(limit) = node.limit {
            ctx.sql.push_str(" limit ");
            ctx.push_usize(limit);
        }
        if let Some(offset) = node.offset {
            ctx.sql.push_str(" offset ");
            ctx.push_usize(offset);
        }
        Ok(())
    }
}
