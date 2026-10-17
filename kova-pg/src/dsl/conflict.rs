use crate::dsl::ExtFunction;
use crate::types::Pg;
use kova_core::dsl::func::Function;
use kova_core::dsl::{ArgAst, IntoArg, Operand};

pub struct Constraint(pub &'static str);

impl<'a> IntoArg<Operand<'a, Pg>, ArgAst> for Constraint {
    #[inline(always)]
    fn into_arg(self) -> Operand<'a, Pg> {
        Operand::from_function(Function::from(ExtFunction::OnConstraint(self.0)))
    }
}
