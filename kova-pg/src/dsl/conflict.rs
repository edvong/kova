use crate::types::Postgres;
use kova_core::dsl::func::{Abs, Function, FunctionBuilder, Length, Lower, Upper};
use kova_core::dsl::{Assignment, Column, IntoArgs};
use kova_core::types::{ColumnMeta, ScalarMarker};

pub struct OnConflict<'a> {
    targets: Vec<ConflictTarget<'a>>,
    assignments: Vec<Assignment<'a, Postgres>>,
}

impl<'a> OnConflict<'a> {
    pub(crate) fn new() -> Self {
        Self {
            targets: Vec::new(),
            assignments: Vec::new(),
        }
    }

    pub fn conflict(&mut self, targets: Vec<ConflictTarget<'a>>) {
        self.targets = targets;
    }

    pub fn ignore(&mut self) {
        self.assignments.clear();
    }

    pub fn update(&mut self, assignments: Vec<Assignment<'a, Postgres>>) {
        self.assignments = assignments;
    }
}

pub struct ConflictTarget<'a> {
    pub(crate) target: TargetExpr<'a>,
}

pub(crate) enum TargetExpr<'a> {
    Column(ColumnMeta),
    Function(Function<'a, Postgres>),
    // Constraint(&'static str) --> ON CONFLICT ON CONSTRAINT "constraint_name"
}

impl<'a> ScalarMarker<Postgres> for ConflictTarget<'a> {}

impl<'a, T> IntoArgs<ConflictTarget<'a>> for Column<'a, T, Postgres> {
    fn size_hint(&self) -> usize { 1 }
    fn append_args(self, out: &mut Vec<ConflictTarget<'a>>) {
        out.push(ConflictTarget {
            target: TargetExpr::Column(self.meta()),
        });
    }
}

macro_rules! impl_conflict_target_args_for_func {
    ($($func:ident),+) => {
        $(
            impl<'a, T> kova_core::dsl::IntoArgs<ConflictTarget<'a>> for $func<Column<'a, T, Postgres>> {
                fn size_hint(&self) -> usize { 1 }
                fn append_args(self, out: &mut Vec<ConflictTarget<'a>>) {
                    out.push(ConflictTarget {
                        target: TargetExpr::Function(self.build_func())
                    });
                }
            }
        )+
    };
}

impl_conflict_target_args_for_func!(Lower, Upper, Abs, Length);
