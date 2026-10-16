use crate::dsl::ExtFunction;
use crate::types::Pg;
use kova_core::dsl::func::Function;
use kova_core::dsl::{IntoArg, IntoOperand, Close, Operand, SingleArgMarker};

pub struct Constraint(pub &'static str);

impl SingleArgMarker for Constraint {}

impl<'a> IntoArg<Operand<'a, Pg>> for Constraint {
    #[inline(always)]
    fn into_arg(self) -> Operand<'a, Pg> {
        Operand::from_function(Function::from(ExtFunction::OnConstraint(self.0)))
    }
}
