use crate::dsl::clause::insert::SourceKind;
use crate::dsl::statement::InsertStatement;
use crate::render::{PlatformRender, Render, RenderContext, RenderData, Renderable};
use crate::types::{Database, Model};
use crate::{Error, Result};

impl<'a, DB: Database, T, X> InsertStatement<'a, DB, T, X> {
    pub(crate) fn render_batches<'s>(&'s self) -> Result<Vec<RenderData<'s, DB>>>
        where
            'a: 's,
            T: Model<DB, X>,
            DB: Renderable<'a, 's> {
        match &self.values.source.inner {
            SourceKind::Select(select_stmt) => {
                let mut ctx = RenderContext::<'a, 's, DB>::new();
                ctx.sql.push_str("insert into ");
                ctx.push_ident(self.target);

                let mut has_cols = false;
                for col in self.values.columns.iter().filter(|c| !c.is_auto) {
                    if !has_cols {
                        ctx.sql.push_str(" (");
                        has_cols = true;
                    } else {
                        ctx.sql.push_str(", ");
                    }
                    ctx.push_ident(col.ident.name);
                }
                if has_cols {
                    ctx.sql.push(')');
                }

                ctx.sql.push(' ');
                select_stmt.render(&mut ctx)?;
                self.render_tail(&mut ctx)?;
                Ok(vec![ctx.into_data()])
            }
            SourceKind::Rows(source_rows) => {
                let rows = source_rows.as_slice();
                if rows.is_empty() {
                    return Ok(Vec::new());
                }

                let cols_per_row = self.values.columns.iter().filter(|c| !c.is_auto).count();
                if cols_per_row == 0 {
                    return Err(Error::QueryError(
                        "INSERT requires at least one column value".into(),
                    ));
                }

                let tail_params = if self.on_conflict.action.is_some() || !self.returning.items.is_empty() {
                    let mut tail_ctx = RenderContext::<'a, 's, DB>::with_capacity(0, 0);
                    self.render_tail(&mut tail_ctx)?;
                    tail_ctx.params.len()
                } else {
                    0
                };

                let max_rows_per_batch = (DB::MAX_PARAMS.saturating_sub(tail_params) / cols_per_row).max(1);
                let mut batches = Vec::with_capacity(rows.len().div_ceil(max_rows_per_batch));
                let mut row_buf: Vec<DB::Value<'s>> = Vec::with_capacity(cols_per_row);

                for chunk in rows.chunks(max_rows_per_batch) {
                    let mut ctx = RenderContext::<'a, 's, DB>::with_capacity(
                        128 + chunk.len() * cols_per_row * 5,
                        chunk.len() * cols_per_row,
                    );
                    ctx.sql.push_str("insert into ");
                    ctx.push_ident(self.target);
                    ctx.sql.push_str(" (");

                    for (i, col) in self.values.columns.iter().filter(|c| !c.is_auto).enumerate() {
                        if i > 0 {
                            ctx.sql.push_str(", ");
                        }
                        ctx.push_ident(col.ident.name);
                    }
                    ctx.sql.push_str(") values ");

                    for (r_idx, row) in chunk.iter().enumerate() {
                        if r_idx > 0 {
                            ctx.sql.push_str(", ");
                        }
                        ctx.sql.push('(');
                        row.write_values(&mut row_buf)?;
                        for (c_idx, val) in row_buf.drain(..).enumerate() {
                            if c_idx > 0 {
                                ctx.sql.push_str(", ");
                            }
                            <DB as PlatformRender<'a, 's, DB::Value<'s>>>::render(val, &mut ctx)?;
                        }
                        ctx.sql.push(')');
                    }

                    self.render_tail(&mut ctx)?;
                    batches.push(ctx.into_data());
                }

                Ok(batches)
            }
        }
    }

    #[inline(always)]
    fn render_tail<'s>(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()>
        where
            'a: 's,
            DB: Renderable<'a, 's> {
        self.on_conflict.render(ctx)?;
        if !self.returning.items.is_empty() {
            ctx.sql.push_str(" returning ");
            self.returning.render(ctx)?;
        }
        Ok(())
    }
}