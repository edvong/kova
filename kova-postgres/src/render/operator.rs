use crate::dsl::ExtOperator;
use crate::dsl::operator::JsonOp;
use crate::types::Postgres;
use kova_kernel::Result;
use kova_kernel::render::{Render, RenderContext};

impl<'a: 's, 's> Render<'a, 's, Postgres> for ExtOperator<'a> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, Postgres>) -> Result<()> {
        let (l, op_str, r) = match self {
            ExtOperator::JsonContains(l, r) => (l, " @> ", r),
            ExtOperator::JsonContainedBy(l, r) => (l, " <@ ", r),
            ExtOperator::JsonHasKey(l, r) => (l, " ? ", r),
            ExtOperator::JsonHasAnyKeys(l, r) => (l, " ?| ", r),
            ExtOperator::JsonHasAllKeys(l, r) => (l, " ?& ", r),
            ExtOperator::JsonPathExists(l, r) => (l, " @? ", r),
            ExtOperator::JsonPathPredicate(l, r) => (l, " @@ ", r),
            ExtOperator::ILike(l, r) => (l, " ilike ", r),
            ExtOperator::NotILike(l, r) => (l, " not ilike ", r),
            ExtOperator::Matches(l, r) => (l, " ~ ", r),
            ExtOperator::NotMatches(l, r) => (l, " !~ ", r),
            ExtOperator::IMatches(l, r) => (l, " ~* ", r),
            ExtOperator::NotIMatches(l, r) => (l, " !~* ", r),
            ExtOperator::ArrayContains(l, r) => (l, " @> ", r),
            ExtOperator::ArrayContainedBy(l, r) => (l, " <@ ", r),
            ExtOperator::ArrayOverlap(l, r) => (l, " && ", r),
        };
        l.render(ctx)?;
        ctx.sql.push_str(op_str);
        r.render(ctx)
    }
}

impl<'a: 's, 's> Render<'a, 's, Postgres> for JsonOp<'a> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, Postgres>) -> Result<()> {
        let (op_str, operand) = match self {
            JsonOp::Path(op) => (" -> ", op),
            JsonOp::PathText(op) => (" ->> ", op),
            JsonOp::PathArray(op) => (" #> ", op),
            JsonOp::PathTextArray(op) => (" #>> ", op),
            JsonOp::Concat(op) => (" || ", op),
            JsonOp::Delete(op) => (" - ", op),
            JsonOp::DeletePath(op) => (" #- ", op),
        };
        ctx.sql.push_str(op_str);
        operand.render(ctx)
    }
}
