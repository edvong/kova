mod operator;
mod pagination;
mod conflict;

use crate::dsl::func::{ExtFunction, ExtSrfFunction};
use crate::dsl::{ExtAssignment, JsonOp, LockMode, Locking, WaitPolicy};
use crate::types::{Postgres, Value};
use kova_core::render::{PlatformRender, Render, RenderContext};
use kova_core::{Error, Result};
use std::any::TypeId;

impl<'a, 's> PlatformRender<'a, 's, Value<'s>> for Postgres {
    #[inline(always)]
    fn render(val: Value<'s>, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()> {
        ctx.params.push(val);
        ctx.sql.push('$');
        ctx.push_usize(ctx.params.len());
        Ok(())
    }
}

impl<'a: 's, 's> PlatformRender<'a, 's, &'s Value<'a>> for Postgres {
    #[inline(always)]
    fn render(val: &'s Value<'a>, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()> {
        <Self as PlatformRender<'a, 's, Value<'s>>>::render(val.as_borrowed(), ctx)
    }
}

impl<'a, 's> Render<'a, 's, Postgres> for ExtAssignment {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, Postgres>) -> Result<()> {
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

impl<'a: 's, 's> Render<'a, 's, Postgres> for ExtFunction<'a> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, Postgres>) -> Result<()> {
        match self {
            ExtFunction::JsonbAgg(op) => {
                ctx.sql.push_str("coalesce(");
                ctx.render_agg("jsonb_agg", |ctx| op.render(ctx))?;
                ctx.sql.push_str(", '[]'::jsonb)");
                Ok(())
            }
            ExtFunction::ArrayAgg(op) => ctx.render_agg("array_agg", |ctx| op.render(ctx)),
            ExtFunction::JsonbBuildObject(projs) => {
                ctx.sql.push_str("jsonb_build_object(");
                for (i, proj) in projs.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(", ");
                    }
                    let key = proj
                        .get_alias()
                        .map(|a| a.name)
                        .or_else(|| proj.get_operand().column_name())
                        .ok_or_else(|| {
                            Error::QueryError(
                                "jsonb_build_object argument requires a column or .alias()".into(),
                            )
                        })?;
                    ctx.push_single_quoted(key);
                    ctx.sql.push_str(", ");
                    proj.get_operand().render(ctx)?;
                }
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::JsonbObject { keys, values } => {
                ctx.sql.push_str("jsonb_object(");
                keys.render(ctx)?;
                if let Some(vals) = values {
                    ctx.sql.push_str(", ");
                    vals.render(ctx)?;
                }
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::RowToJson(op) => {
                ctx.sql.push_str("row_to_json(");
                op.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::JsonChain { base, op } => {
                // In PostgreSQL, `-` (JsonOp::Delete) has higher precedence than `->`, `#>`, `||`, and `#-`.
                // Wrap composite base expressions in parentheses to preserve left-to-right evaluation.
                let wrap_parens = matches!(&**op, JsonOp::Delete(_))
                    && base.is_composite();
                if wrap_parens {
                    ctx.sql.push('(');
                }
                base.render(ctx)?;
                if wrap_parens {
                    ctx.sql.push(')');
                }
                op.render(ctx)
            }
            ExtFunction::Any(op) => {
                ctx.sql.push_str("any(");
                op.render_bare(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::All(op) => {
                ctx.sql.push_str("all(");
                op.render_bare(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::Array(ops) => {
                ctx.sql.push_str("array[");
                for (i, op) in ops.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(", ");
                    }
                    op.render(ctx)?;
                }
                ctx.sql.push(']');
                Ok(())
            }
            ExtFunction::RowNumber => {
                ctx.sql.push_str("row_number()");
                Ok(())
            }
            ExtFunction::Rank => {
                ctx.sql.push_str("rank()");
                Ok(())
            }
            ExtFunction::DenseRank => {
                ctx.sql.push_str("dense_rank()");
                Ok(())
            }
            ExtFunction::PercentRank => {
                ctx.sql.push_str("percent_rank()");
                Ok(())
            }
            ExtFunction::CumeDist => {
                ctx.sql.push_str("cume_dist()");
                Ok(())
            }
            ExtFunction::Ntile(op) => {
                ctx.sql.push_str("ntile(");
                op.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::Lag { expr, offset, default } => {
                ctx.sql.push_str("lag(");
                expr.render(ctx)?;
                if let Some(off) = offset {
                    ctx.sql.push_str(", ");
                    off.render(ctx)?;
                }
                if let Some(def) = default {
                    ctx.sql.push_str(", ");
                    def.render(ctx)?;
                }
                ctx.sql.push(')');
                Ok(())
            }
            ExtFunction::Lead { expr, offset, default } => {
                ctx.sql.push_str("lead(");
                expr.render(ctx)?;
                if let Some(off) = offset {
                    ctx.sql.push_str(", ");
                    off.render(ctx)?;
                }
                if let Some(def) = default {
                    ctx.sql.push_str(", ");
                    def.render(ctx)?;
                }
                ctx.sql.push(')');
                Ok(())
            }
        }
    }
}

impl<'a: 's, 's> PlatformRender<'a, 's, (&'s ExtSrfFunction<'a>, Option<TypeId>)> for Postgres {
    fn render(
        (srf, tag): (&'s ExtSrfFunction<'a>, Option<TypeId>), ctx: &mut RenderContext<'a, 's, Self>,
    ) -> Result<()> {
        match srf {
            ExtSrfFunction::Unnest(op) => {
                ctx.sql.push_str("unnest(");
                op.render(ctx)?;
                ctx.sql.push(')');
                ctx.push_srf_alias("unnest", tag, Some("unnest"));
                Ok(())
            }
            ExtSrfFunction::JsonbArrayElements(op) => {
                ctx.sql.push_str("jsonb_array_elements(");
                op.render(ctx)?;
                ctx.sql.push(')');
                ctx.push_srf_alias("jsonb_array_elements", tag, None);
                Ok(())
            }
            ExtSrfFunction::JsonbEach(op) => {
                ctx.sql.push_str("jsonb_each(");
                op.render(ctx)?;
                ctx.sql.push(')');
                ctx.push_srf_alias("jsonb_each", tag, None);
                Ok(())
            }
            ExtSrfFunction::JsonbEachText(op) => {
                ctx.sql.push_str("jsonb_each_text(");
                op.render(ctx)?;
                ctx.sql.push(')');
                ctx.push_srf_alias("jsonb_each_text", tag, None);
                Ok(())
            }
            ExtSrfFunction::GenerateSeries { start, stop, step } => {
                ctx.sql.push_str("generate_series(");
                start.render(ctx)?;
                ctx.sql.push_str(", ");
                stop.render(ctx)?;
                if let Some(s) = step {
                    ctx.sql.push_str(", ");
                    s.render(ctx)?;
                }
                ctx.sql.push(')');
                ctx.push_srf_alias("generate_series", tag, Some("generate_series"));
                Ok(())
            }
        }
    }
}

impl<'a, 's> Render<'a, 's, Postgres> for Locking {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, Postgres>) -> Result<()> {
        for item in &self.locks {
            match item.mode {
                LockMode::Update => ctx.sql.push_str(" for update"),
                LockMode::NoKeyUpdate => ctx.sql.push_str(" for no key update"),
                LockMode::Share => ctx.sql.push_str(" for share"),
                LockMode::KeyShare => ctx.sql.push_str(" for key share"),
            }
            if !item.tables.is_empty() {
                ctx.sql.push_str(" of ");
                for (i, tbl) in item.tables.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(", ");
                    }
                    ctx.push_ident(tbl);
                }
            }
            if let Some(wait) = &item.wait {
                match wait {
                    WaitPolicy::NoWait => ctx.sql.push_str(" nowait"),
                    WaitPolicy::SkipLocked => ctx.sql.push_str(" skip locked"),
                }
            }
        }
        Ok(())
    }
}
