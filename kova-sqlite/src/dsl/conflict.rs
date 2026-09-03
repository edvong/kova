use crate::types::Sqlite;
use kova_core::dsl::func::{Abs, Function, FunctionBuilder, Length, Lower, Upper};
use kova_core::dsl::{Assignment, Column, IntoArg, SingleArgMarker};
use kova_core::types::ColumnMeta;

pub struct OnConflict<'a> {
    targets: Vec<ConflictTarget<'a>>,
    assignments: Vec<Assignment<'a, Sqlite>>,
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

    pub fn clear(&mut self) {
        self.assignments.clear();
    }

    pub fn set(&mut self, assignments: Vec<Assignment<'a, Sqlite>>) {
        self.assignments.clear();
        self.assignments = assignments;
    }
}

pub struct ConflictTarget<'a> {
    pub(crate) target: TargetExpr<'a>,
}

pub(crate) enum TargetExpr<'a> {
    Column(ColumnMeta),
    Function(Function<'a, Sqlite>),
    // Constraint(&'static str) -- > ON CONFLICT ON CONSTRAINT "constraint_name"
}

impl<'a> SingleArgMarker for ConflictTarget<'a> {}

impl<'a, T> IntoArg<ConflictTarget<'a>> for Column<'a, T, Sqlite> {
    fn into_arg(self) -> ConflictTarget<'a> {
        ConflictTarget {
            target: TargetExpr::Column(self.meta()),
        }
    }
}

macro_rules! impl_conflict_target_for_func {
    ($($func:ident),+) => {
        $(
            impl<'a, T> IntoArg<ConflictTarget<'a>> for $func<Column<'a, T, Sqlite>> {
                #[inline(always)]
                fn into_arg(self) -> ConflictTarget<'a> {
                    ConflictTarget {
                        target: TargetExpr::Function(self.build_func())
                    }
                }
            }
        )+
    };
}

impl_conflict_target_for_func!(Lower, Upper, Abs, Length);
