mod conflict;

use crate::dsl::func::ExtSrfFunction;
use crate::dsl::{ExtAssignment, ExtFunction, ExtOperator};
use crate::types::{Sqlite, Value};
use kova_core::dsl::{PaginationClause, Projection};
use kova_core::render::{PlatformRender, Render, RenderContext};
use kova_core::{Error, Result};
use std::any::TypeId;
use std::fmt::Write;

impl<'a, 's> PlatformRender<'a, 's, Value<'s>> for Sqlite {
    #[inline(always)]
    fn render(val: Value<'s>, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()> {
        ctx.params.push(val);
        ctx.sql.push('?');
        Ok(())
    }
}

impl<'a: 's, 's> PlatformRender<'a, 's, &'s Value<'a>> for Sqlite {
    #[inline(always)]
    fn render(val: &'s Value<'a>, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()> {
        <Self as PlatformRender<'a, 's, Value<'s>>>::render(val.as_borrowed(), ctx)
    }
}

impl<'a, 's> PlatformRender<'a, 's, &'s PaginationClause> for Sqlite {
    #[inline(always)]
    fn render(node: &'s PaginationClause, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()> {
        match (node.limit, node.offset) {
            (Some(limit), Some(offset)) => {
                ctx.sql.push_str(" limit ");
                ctx.push_usize(limit);
                ctx.sql.push_str(" offset ");
                ctx.push_usize(offset);
            }
            (Some(limit), None) => {
                ctx.sql.push_str(" limit ");
                ctx.push_usize(limit);
            }
            (None, Some(offset)) => {
                ctx.sql.push_str(" limit -1 offset ");
                ctx.push_usize(offset);
            }
            (None, None) => {}
        }
        Ok(())
    }
}

impl<'a: 's, 's> Render<'a, 's, Sqlite> for ExtOperator<'a> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, Sqlite>) -> Result<()> {
        match self {
            ExtOperator::Match(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" match ");
                r.render(ctx)
            }
            ExtOperator::Regexp(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" regexp ");
                r.render(ctx)
            }
        }
    }
}

impl<'a, 's> Render<'a, 's, Sqlite> for ExtAssignment {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, Sqlite>) -> Result<()> {
        match self {
            ExtAssignment::ExcludedColumn(meta) => {
                ctx.push_ident(meta.name);
                ctx.sql.push_str(" = excluded.");
                ctx.push_ident(meta.name);
                Ok(())
            }
        }
    }
}

fn render_sqlite_json_object<'a: 's, 's>(
    func_name: &'static str, projs: &'s [Projection<'a, Sqlite, ()>],
    ctx: &mut RenderContext<'a, 's, Sqlite>,
) -> Result<()> {
    ctx.sql.push_str(func_name);
    ctx.sql.push('(');
    for (i, proj) in projs.iter().enumerate() {
        if i > 0 {
            ctx.sql.push_str(", ");
        }
        let key = proj
            .get_alias()
            .map(|a| a.name)
            .or_else(|| proj.get_operand().column_name())
            .ok_or_else(|| {
                Error::QueryError(format!("{func_name} argument requires a column or .alias()"))
            })?;
        ctx.push_single_quoted(key);
        ctx.sql.push_str(", ");
        proj.get_operand().render(ctx)?;
    }
    ctx.sql.push(')');
    Ok(())
}

impl<'a: 's, 's> Render<'a, 's, Sqlite> for ExtFunction<'a> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, Sqlite>) -> Result<()> {
        match self {
            ExtFunction::GroupConcat(op, sep) => ctx.render_agg("group_concat", |ctx| {
                op.render(ctx)?;
                if let Some(s) = sep {
                    ctx.sql.push_str(", ");
                    ctx.push_single_quoted(s);
                }
                Ok(())
            }),
            ExtFunction::JsonGroupArray(op) => {
                ctx.render_agg("json_group_array", |ctx| op.render(ctx))
            }
            ExtFunction::JsonbGroupArray(op) => {
                ctx.render_agg("jsonb_group_array", |ctx| op.render(ctx))
            }
            ExtFunction::JsonObject(projs) => render_sqlite_json_object("json_object", projs, ctx),
            ExtFunction::JsonbObject(projs) => {
                render_sqlite_json_object("jsonb_object", projs, ctx)
            }
            ExtFunction::JsonExtract(base, path) => {
                ctx.sql.push_str("json_extract(");
                base.render(ctx)?;
                ctx.sql.push_str(", ");
                path.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::JsonbExtract(base, path) => {
                ctx.sql.push_str("jsonb_extract(");
                base.render(ctx)?;
                ctx.sql.push_str(", ");
                path.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::JsonValid(op) => {
                ctx.sql.push_str("json_valid(");
                op.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::JsonType(base, path) => {
                ctx.sql.push_str("json_type(");
                base.render(ctx)?;
                ctx.sql.push_str(", ");
                path.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::Json(op) => {
                ctx.sql.push_str("json(");
                op.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::Bm25(table, weights) => {
                ctx.sql.push_str("bm25(");
                ctx.push_ident(table);
                for w in weights {
                    let _ = write!(ctx.sql, ", {w}");
                }
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::Highlight(table, col_idx, left, right) => {
                ctx.sql.push_str("highlight(");
                ctx.push_ident(table);
                ctx.sql.push_str(", ");
                ctx.push_i32(*col_idx);
                ctx.sql.push_str(", ");
                left.render(ctx)?;
                ctx.sql.push_str(", ");
                right.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::Snippet(table, col_idx, left, right, ellipsis, max_tokens) => {
                ctx.sql.push_str("snippet(");
                ctx.push_ident(table);
                ctx.sql.push_str(", ");
                ctx.push_i32(*col_idx);
                ctx.sql.push_str(", ");
                left.render(ctx)?;
                ctx.sql.push_str(", ");
                right.render(ctx)?;
                ctx.sql.push_str(", ");
                ellipsis.render(ctx)?;
                ctx.sql.push_str(", ");
                ctx.push_i32(*max_tokens);
                ctx.sql.push(')');
                Ok(())
            }
        }
    }
}

impl<'a: 's, 's> PlatformRender<'a, 's, (&'s ExtSrfFunction<'a>, Option<TypeId>)> for Sqlite {
    fn render(
        (srf, tag): (&'s ExtSrfFunction<'a>, Option<TypeId>), ctx: &mut RenderContext<'a, 's, Self>,
    ) -> Result<()> {
        match srf {
            ExtSrfFunction::JsonEach(base, path) => {
                ctx.sql.push_str("json_each(");
                base.render(ctx)?;
                if let Some(p) = path {
                    ctx.sql.push_str(", ");
                    p.render(ctx)?;
                }
                ctx.sql.push(')');
                ctx.push_srf_alias("json_each", tag, None);
                Ok(())
            }
            ExtSrfFunction::JsonTree(base, path) => {
                ctx.sql.push_str("json_tree(");
                base.render(ctx)?;
                if let Some(p) = path {
                    ctx.sql.push_str(", ");
                    p.render(ctx)?;
                }
                ctx.sql.push(')');
                ctx.push_srf_alias("json_tree", tag, None);
                Ok(())
            }
        }
    }
}