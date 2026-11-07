use crate::dsl::Constraint;
use crate::types::Postgres;
use kova_kernel::Result;
use kova_kernel::dsl::component::ComponentKind;
use kova_kernel::dsl::{ConflictAction, OnConflictClause};
use kova_kernel::render::{PlatformRender, Render, RenderContext};

impl<'a: 's, 's> PlatformRender<'a, 's, &'s OnConflictClause<'a, Postgres>> for Postgres {
    fn render(clause: &'s OnConflictClause<'a, Postgres>, ctx: &mut RenderContext<'a, 's, Self>)
        -> Result<()> {
        let Some(action) = &clause.action else {
            return Ok(());
        };

        ctx.sql.push_str(" on conflict");
        if let Some(target) = &clause.target {
            match &target.inner {
                ComponentKind::Standard(cols) => {
                    if !cols.is_empty() {
                        ctx.sql.push_str(" (");
                        let prev_unqualify = ctx.unqualify_columns;
                        ctx.unqualify_columns = true;
                        for (i, col) in cols.iter().enumerate() {
                            if i > 0 {
                                ctx.sql.push_str(", ");
                            }
                            col.render(ctx)?;
                        }
                        ctx.unqualify_columns = prev_unqualify;
                        ctx.sql.push(')');
                    }
                }
                ComponentKind::Extension(Constraint(name)) => {
                    ctx.sql.push_str(" on constraint ");
                    ctx.push_ident(name);
                }
            }
        }

        match action {
            ConflictAction::DoNothing => {
                ctx.sql.push_str(" do nothing");
                Ok(())
            }
            ConflictAction::DoUpdate { assignments, r#where } => {
                ctx.sql.push_str(" do update set ");
                for (i, item) in assignments.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(", ");
                    }
                    item.render(ctx)?;
                }
                r#where.render(ctx)
            }
        }
    }
}