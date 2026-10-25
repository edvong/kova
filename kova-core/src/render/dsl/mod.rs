mod component;
mod operator;

use crate::Result;
use crate::dsl::clause::join::JoinConstraintKind;
use crate::dsl::clause::order::OrderOperand;
use crate::dsl::clause::table::TableInner;
use crate::dsl::clause::{
    AssignmentClause, CombineClause, DistinctClause, FromItem, GroupBy, GroupByClause,
    HavingClause, Join, JoinClause, JoinType, LockingClause, OnConflictClause, OrderByClause,
    Ordering, PaginationClause, Projection, ProjectionClause, SetOperator, TableClause,
    WhereClause, WithClause,
};
use crate::dsl::func::{FrameBound, FrameType, Function, PartitionBy, StandardFunction, Window};
use crate::dsl::operand::OperandNode;
use crate::dsl::{
    Assignment, Conflict, Direction, MathOperator, NullsOrder, Operand, StandardAssignment,
};
use crate::render::{PlatformRender, Render, RenderContext, RenderDispatch, Renderable};
use crate::types::{Database, Extension};
use std::any::TypeId;

impl<'a, DB: Database> Operand<'a, DB> {
    // For operators with outer parentheses (e.g. `IN (...) `, `ANY(...) `, `ALL(...) `)
    // to avoid double parentheses in subqueries.
    #[inline(always)]
    pub fn render_bare<'s>(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()>
        where
            'a: 's,
            DB: Renderable<'a, 's> {
        match &self.inner {
            OperandNode::Subselect(sub) => sub.render(ctx),
            _ => self.render(ctx),
        }
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for Operand<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match &self.inner {
            OperandNode::Value(val) => {
                <DB as PlatformRender<'a, 's, &'s DB::Value<'a>>>::render(val, ctx)
            }
            OperandNode::Column(meta) => {
                ctx.push_column(*meta);
                Ok(())
            }
            OperandNode::Table(meta) => {
                ctx.push_table_ref(*meta);
                Ok(())
            }
            OperandNode::Function(func) => func.render(ctx),
            OperandNode::Subselect(sub) => {
                ctx.sql.push('(');
                sub.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            OperandNode::Wildcard => {
                ctx.sql.push('*');
                Ok(())
            }
            OperandNode::Math(op, left, right) => {
                ctx.sql.push('(');
                left.render(ctx)?;
                match op {
                    MathOperator::Add => ctx.sql.push_str(" + "),
                    MathOperator::Sub => ctx.sql.push_str(" - "),
                    MathOperator::Mul => ctx.sql.push_str(" * "),
                    MathOperator::Div => ctx.sql.push_str(" / "),
                }
                right.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            OperandNode::Error(err) => Err((**err).clone()),
        }
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for Function<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        let prev_mods = ctx.agg_modifiers.replace((
            self.quantifier,
            &self.order_by,
            self.filter.as_deref(),
            self.window.as_deref(),
        ));
        let res = self.inner.render(ctx);
        let unconsumed = ctx.agg_modifiers.take();
        ctx.agg_modifiers = prev_mods;
        res?;

        if let Some((_, _, filter, window)) = unconsumed {
            if let Some(cond) = filter {
                ctx.sql.push_str(" filter (where ");
                cond.render(ctx)?;
                ctx.sql.push(')');
            }
            if let Some(win) = window {
                ctx.sql.push_str(" over (");
                win.render(ctx)?;
                ctx.sql.push(')');
            }
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for StandardFunction<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match self {
            StandardFunction::Count(op) => ctx.render_agg("count", |ctx| op.render(ctx)),
            StandardFunction::Sum(op) => ctx.render_agg("sum", |ctx| op.render(ctx)),
            StandardFunction::Max(op) => ctx.render_agg("max", |ctx| op.render(ctx)),
            StandardFunction::Min(op) => ctx.render_agg("min", |ctx| op.render(ctx)),
            StandardFunction::Avg(op) => ctx.render_agg("avg", |ctx| op.render(ctx)),
            StandardFunction::Length(op) => {
                ctx.sql.push_str("length(");
                op.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            StandardFunction::Lower(op) => {
                ctx.sql.push_str("lower(");
                op.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            StandardFunction::Upper(op) => {
                ctx.sql.push_str("upper(");
                op.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            StandardFunction::Abs(op) => {
                ctx.sql.push_str("abs(");
                op.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
        }
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for PartitionBy<'a, DB, X> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        self.operand.render(ctx)
    }
}

fn render_frame_bound<'a, 's, DB: crate::types::Database>(
    bound: FrameBound, ctx: &mut RenderContext<'a, 's, DB>,
) {
    match bound {
        FrameBound::UnboundedPreceding => ctx.sql.push_str("unbounded preceding"),
        FrameBound::Preceding(n) => {
            ctx.push_usize(n);
            ctx.sql.push_str(" preceding");
        }
        FrameBound::CurrentRow => ctx.sql.push_str("current row"),
        FrameBound::Following(n) => {
            ctx.push_usize(n);
            ctx.sql.push_str(" following");
        }
        FrameBound::UnboundedFollowing => ctx.sql.push_str("unbounded following"),
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for Window<'a, DB, X> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        let mut has_prev = false;

        if !self.partition_by.is_empty() {
            ctx.sql.push_str("partition by ");
            for (i, item) in self.partition_by.iter().enumerate() {
                if i > 0 {
                    ctx.sql.push_str(", ");
                }
                item.render(ctx)?;
            }
            has_prev = true;
        }

        if !self.order_by.is_empty() {
            if has_prev {
                ctx.sql.push(' ');
            }
            ctx.sql.push_str("order by ");
            for (i, item) in self.order_by.iter().enumerate() {
                if i > 0 {
                    ctx.sql.push_str(", ");
                }
                item.render(ctx)?;
            }
            has_prev = true;
        }

        if let Some(frame) = self.frame {
            if has_prev {
                ctx.sql.push(' ');
            }
            match frame.typ {
                FrameType::Rows => ctx.sql.push_str("rows "),
                FrameType::Range => ctx.sql.push_str("range "),
                FrameType::Groups => ctx.sql.push_str("groups "),
            }
            if let Some(end) = frame.end {
                ctx.sql.push_str("between ");
                render_frame_bound(frame.start, ctx);
                ctx.sql.push_str(" and ");
                render_frame_bound(end, ctx);
            } else {
                render_frame_bound(frame.start, ctx);
            }
        }

        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for Assignment<'a, DB, X> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        self.inner.render(ctx)
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for StandardAssignment<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match self {
            StandardAssignment::Value(meta, op) => {
                ctx.push_ident(meta.name);
                ctx.sql.push_str(" = ");
                op.render(ctx)
            }
            StandardAssignment::Increment(meta, op) => {
                ctx.push_ident(meta.name);
                ctx.sql.push_str(" = ");
                ctx.push_column(*meta);
                ctx.sql.push_str(" + ");
                op.render(ctx)
            }
            StandardAssignment::Decrement(meta, op) => {
                ctx.push_ident(meta.name);
                ctx.sql.push_str(" = ");
                ctx.push_column(*meta);
                ctx.sql.push_str(" - ");
                op.render(ctx)
            }
        }
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for AssignmentClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                ctx.sql.push_str(", ");
            }
            item.render(ctx)?;
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for Projection<'a, DB, X> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        self.operand.render(ctx)?;
        if let Some(alias) = self.alias {
            ctx.sql.push_str(" as ");
            ctx.push_ident(alias.name);
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for ProjectionClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        if self.items.is_empty() {
            ctx.sql.push('*');
            return Ok(());
        }
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                ctx.sql.push_str(", ");
            }
            item.render(ctx)?;
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for FromItem<'a, DB, X> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match &self.inner {
            TableInner::Table(name, tag) => {
                ctx.push_table_def(name, *tag);
                Ok(())
            }
            TableInner::Function(func, tag) => <DB as PlatformRender<
                'a,
                's,
                (&'s <<DB as Database>::Extension as Extension>::TableFunction<'a>, Option<TypeId>),
            >>::render((func, *tag), ctx),
        }
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for TableClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        if self.items.is_empty() {
            return Ok(());
        }
        ctx.sql.push_str(" from ");
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                ctx.sql.push_str(", ");
            }
            item.render(ctx)?;
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for Join<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match self.typ {
            JoinType::Inner => ctx.sql.push_str(" inner join "),
            JoinType::Left => ctx.sql.push_str(" left join "),
            JoinType::Right => ctx.sql.push_str(" right join "),
            JoinType::Full => ctx.sql.push_str(" full join "),
            JoinType::Cross => ctx.sql.push_str(" cross join "),
        }
        ctx.push_table_def(self.target_table, self.tag);

        match &self.constraint.kind {
            JoinConstraintKind::On(cond) => {
                ctx.sql.push_str(" on ");
                cond.render(ctx)
            }
            JoinConstraintKind::Using(cols) => {
                ctx.sql.push_str(" using (");
                for (i, col) in cols.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(", ");
                    }
                    ctx.push_ident(col.name);
                }
                ctx.sql.push(')');
                Ok(())
            }
            JoinConstraintKind::None => Ok(()),
        }
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for JoinClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        for join in &self.items {
            join.render(ctx)?;
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for WhereClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        if let Some(cond) = &self.cond {
            ctx.sql.push_str(" where ");
            cond.render(ctx)?;
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for GroupBy<'a, DB, X> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        self.operand.render(ctx)
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for GroupByClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        if self.items.is_empty() {
            return Ok(());
        }
        ctx.sql.push_str(" group by ");
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                ctx.sql.push_str(", ");
            }
            item.render(ctx)?;
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for HavingClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        if let Some(cond) = &self.cond {
            ctx.sql.push_str(" having ");
            cond.render(ctx)?;
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for Ordering<'a, DB, X> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match &self.operand {
            OrderOperand::Operand(op) => op.render(ctx)?,
            OrderOperand::Condition(cond) => cond.render(ctx)?,
        }
        if let Some(dir) = &self.direction {
            match dir {
                Direction::Asc => ctx.sql.push_str(" asc"),
                Direction::Desc => ctx.sql.push_str(" desc"),
            }
        }
        if let Some(nulls) = &self.nulls {
            match nulls {
                NullsOrder::First => ctx.sql.push_str(" nulls first"),
                NullsOrder::Last => ctx.sql.push_str(" nulls last"),
            }
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for [Ordering<'a, DB, ()>] {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        if !self.is_empty() {
            ctx.sql.push_str(" order by ");
            for (i, item) in self.iter().enumerate() {
                if i > 0 {
                    ctx.sql.push_str(", ");
                }
                item.render(ctx)?;
            }
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for OrderByClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        self.items.as_slice().render(ctx)
    }
}

impl<'a, 's, DB: crate::types::Database> Render<'a, 's, DB> for DistinctClause {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        if self.distinct {
            ctx.sql.push_str("distinct ");
        }
        Ok(())
    }
}

impl<'a, 's, DB> Render<'a, 's, DB> for PaginationClause
    where
        DB: PlatformRender<'a, 's, &'s PaginationClause> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        <DB as PlatformRender<'a, 's, &'s PaginationClause>>::render(self, ctx)
    }
}

impl<'a, 's, DB> Render<'a, 's, DB> for LockingClause<DB>
    where
        DB: Database + RenderDispatch<'a, 's, <<DB as Database>::Extension as Extension>::Locking> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        DB::dispatch(&self.inner, ctx)
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for WithClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        if self.items.is_empty() {
            return Ok(());
        }
        ctx.sql.push_str("with ");
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                ctx.sql.push_str(", ");
            }
            ctx.push_ident(item.name);
            ctx.sql.push_str(" as (");
            item.statement.render(ctx)?;
            ctx.sql.push(')');
        }
        ctx.sql.push(' ');
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for CombineClause<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        for item in &self.items {
            match item.operator {
                SetOperator::Union => ctx.sql.push_str(" union "),
                SetOperator::UnionAll => ctx.sql.push_str(" union all "),
                SetOperator::Intersect => ctx.sql.push_str(" intersect "),
                SetOperator::Except => ctx.sql.push_str(" except "),
            }
            item.statement.render(ctx)?;
        }
        Ok(())
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for Conflict<'a, DB, X> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        self.operand.render(ctx)
    }
}

// Delegate OnConflictClause rendering entirely to the platform layer.
impl<'a: 's, 's, DB> Render<'a, 's, DB> for OnConflictClause<'a, DB>
    where
        DB: PlatformRender<'a, 's, &'s OnConflictClause<'a, DB>> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        <DB as PlatformRender<'a, 's, &'s OnConflictClause<'a, DB>>>::render(self, ctx)
    }
}
