use crate::types::Postgres;
use kova_core::dsl::func::{Abs, Function, Length, Lower, Upper};
use kova_core::dsl::{Assignment, Column, IntoArg, SingleArgMarker};
use kova_core::types::IdentifierMeta;

#[derive(Default)]
pub struct OnConflict<'a> {
    items: Vec<ConflictTarget<'a>>,
    assignments: Vec<Assignment<'a, Postgres, ()>>,
}

impl<'a> OnConflict<'a> {
    pub fn set_conflict(&mut self, args: Vec<ConflictTarget<'a>>) {
        self.items = args;
    }

    pub fn set_ignore(&mut self) {
        self.assignments.clear();
    }

    pub fn set_assignment(&mut self, assignments: Vec<Assignment<'a, Postgres, ()>>) {
        self.assignments = assignments;
    }
}

pub struct ConflictTarget<'a> {
    pub(crate) target: TargetExpr<'a>,
}

pub(crate) enum TargetExpr<'a> {
    Column(IdentifierMeta),
    Function(Function<'a, Postgres>),
    Constraint(&'static str), // ON CONFLICT ON CONSTRAINT "constraint_name"
}

pub struct Constraint(pub &'static str);

impl SingleArgMarker for Constraint {}

impl<'a> IntoArg<ConflictTarget<'a>> for Constraint {
    #[inline(always)]
    fn into_arg(self) -> ConflictTarget<'a> {
        ConflictTarget { target: TargetExpr::Constraint(self.0) }
    }
}

impl<'a> SingleArgMarker for ConflictTarget<'a> {}

impl<'a, T> IntoArg<ConflictTarget<'a>> for Column<T, Postgres> {
    #[inline(always)]
    fn into_arg(self) -> ConflictTarget<'a> {
        ConflictTarget { target: TargetExpr::Column(self.identifier()) }
    }
}

macro_rules! impl_conflict_target_args_for_func {
    ($($func:ident),+) => {
        $(
            impl<'a, E> kova_core::dsl::IntoArg<ConflictTarget<'a>> for $func<E>
                where
                    $func<E>: kova_core::dsl::OpNode,
                    E: kova_core::dsl::OpNode + kova_core::dsl::IntoOperand<'a, <$func<E> as kova_core::dsl::OpNode>::Input, Postgres, kova_core::dsl::OpAst, ()> {
                #[inline(always)]
                fn into_arg(self) -> ConflictTarget<'a> {
                    ConflictTarget { target: TargetExpr::Function(self.into_func()) }
                }
            }
        )+
    };
}

impl_conflict_target_args_for_func!(Lower, Upper, Abs, Length);
