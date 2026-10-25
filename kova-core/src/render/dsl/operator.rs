use crate::Result;
use crate::dsl::condition::ConditionNode;
use crate::dsl::{Condition, Operator, StandardOperator};
use crate::render::{Render, RenderContext, Renderable};

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for Operator<'a, DB> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        self.inner.render(ctx)
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for StandardOperator<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match self {
            StandardOperator::Eq(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" = ");
                r.render(ctx)
            }
            StandardOperator::Ne(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" != ");
                r.render(ctx)
            }
            StandardOperator::Gt(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" > ");
                r.render(ctx)
            }
            StandardOperator::Ge(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" >= ");
                r.render(ctx)
            }
            StandardOperator::Lt(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" < ");
                r.render(ctx)
            }
            StandardOperator::Le(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" <= ");
                r.render(ctx)
            }
            StandardOperator::IsNull(op) => {
                op.render(ctx)?;
                ctx.sql.push_str(" is null");
                Ok(())
            }
            StandardOperator::IsNotNull(op) => {
                op.render(ctx)?;
                ctx.sql.push_str(" is not null");
                Ok(())
            }
            StandardOperator::Between(op, low, high) => {
                op.render(ctx)?;
                ctx.sql.push_str(" between ");
                low.render(ctx)?;
                ctx.sql.push_str(" and ");
                high.render(ctx)
            }
            StandardOperator::NotBetween(op, low, high) => {
                op.render(ctx)?;
                ctx.sql.push_str(" not between ");
                low.render(ctx)?;
                ctx.sql.push_str(" and ");
                high.render(ctx)
            }
            StandardOperator::InList(op, list) => {
                if list.is_empty() {
                    ctx.sql.push_str("false");
                    return Ok(());
                }
                op.render(ctx)?;
                ctx.sql.push_str(" in (");
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(", ");
                    }
                    item.render(ctx)?;
                }
                ctx.sql.push(')');
                Ok(())
            }
            StandardOperator::NotInList(op, list) => {
                if list.is_empty() {
                    ctx.sql.push_str("true");
                    return Ok(());
                }
                op.render(ctx)?;
                ctx.sql.push_str(" not in (");
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(", ");
                    }
                    item.render(ctx)?;
                }
                ctx.sql.push(')');
                Ok(())
            }
            StandardOperator::InSelect(op, sub) => {
                op.render(ctx)?;
                ctx.sql.push_str(" in (");
                sub.render_bare(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            StandardOperator::NotInSelect(op, sub) => {
                op.render(ctx)?;
                ctx.sql.push_str(" not in (");
                sub.render_bare(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
            StandardOperator::Like(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" like ");
                r.render(ctx)
            }
            StandardOperator::NotLike(l, r) => {
                l.render(ctx)?;
                ctx.sql.push_str(" not like ");
                r.render(ctx)
            }
        }
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>, X> Render<'a, 's, DB> for Condition<'a, DB, X> {
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        self.node.render(ctx)
    }
}

impl<'a: 's, 's, DB: Renderable<'a, 's>> Render<'a, 's, DB> for ConditionNode<'a, DB> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match self {
            ConditionNode::Op(op) => op.render(ctx),
            ConditionNode::And(nodes) => {
                for (i, node) in nodes.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(" and ");
                    }
                    let is_or = matches!(node, ConditionNode::Or(_));
                    if is_or {
                        ctx.sql.push('(');
                    }
                    node.render(ctx)?;
                    if is_or {
                        ctx.sql.push(')');
                    }
                }
                Ok(())
            }
            ConditionNode::Or(nodes) => {
                for (i, node) in nodes.iter().enumerate() {
                    if i > 0 {
                        ctx.sql.push_str(" or ");
                    }
                    let is_and = matches!(node, ConditionNode::And(_));
                    if is_and {
                        ctx.sql.push('(');
                    }
                    node.render(ctx)?;
                    if is_and {
                        ctx.sql.push(')');
                    }
                }
                Ok(())
            }
            ConditionNode::Not(inner) => {
                ctx.sql.push_str("not (");
                inner.render(ctx)?;
                ctx.sql.push(')');
                Ok(())
            }
        }
    }
}