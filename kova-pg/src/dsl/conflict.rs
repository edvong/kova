use crate::dsl::func::ExtFunction;
use crate::types::Pg;
use kova_core::dsl::func::Function;
use kova_core::dsl::{ArgAst, Conflict, IntoArg, IntoConflict, Operand};

pub struct Constraint(pub &'static str);

impl<'a, X> IntoConflict<'a, Pg, X> for Constraint {
    #[inline(always)]
    fn into_conflict(self) -> Conflict<'a, Pg, X> {
        Conflict::new(Operand::from_function(Function::from(ExtFunction::OnConstraint(self.0))))
    }
}

impl<'a, X> IntoArg<Conflict<'a, Pg, X>, ArgAst> for Constraint {
    #[inline(always)]
    fn into_arg(self) -> Conflict<'a, Pg, X> {
        self.into_conflict()
    }
}
