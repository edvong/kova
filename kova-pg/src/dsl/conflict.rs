use crate::types::Postgres;
use kova_core::dsl::func::{Abs, BuildFunction, Function, Length, Lower, Upper};
use kova_core::dsl::{Assignment, Column, IntoArgs};
use kova_core::types::ColumnMeta;

pub struct OnConflict<'a> {
    items: Vec<ConflictTarget<'a>>,
    assignments: Vec<Assignment<'a, Postgres, ()>>,
}

impl<'a> OnConflict<'a> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new(), assignments: Vec::new() }
    }

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
    Column(ColumnMeta),
    Function(Function<'a, Postgres>),
    // Constraint(&'static str) --> ON CONFLICT ON CONSTRAINT "constraint_name"
}

impl<'a, T> IntoArgs<ConflictTarget<'a>> for Column<'a, T, Postgres> {
    #[inline(always)]
    fn size_hint(&self) -> usize {
        1
    }

    #[inline(always)]
    fn for_each_arg<F>(self, f: &mut F)
        where
            F: FnMut(ConflictTarget<'a>) {
        f(ConflictTarget { target: TargetExpr::Column(self.meta()) });
    }
}

macro_rules! impl_conflict_target_args_for_func {
    ($($func:ident),+) => {
        $(
            impl<'a, T> kova_core::dsl::IntoArgs<ConflictTarget<'a>> for $func<Column<'a, T, Postgres>> {
                #[inline(always)]
                fn size_hint(&self) -> usize { 1 }

                #[inline(always)]
                fn for_each_arg<F>(self, f: &mut F) where F: FnMut(ConflictTarget<'a>) {
                    f(ConflictTarget {
                        target: TargetExpr::Function(self.build_func())
                    });
                }
            }
        )+
    };
}

impl_conflict_target_args_for_func!(Lower, Upper, Abs, Length);
