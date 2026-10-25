use std::any::TypeId;
use crate::dsl::{ArgAst, Condition, EraseX, IntoArg};
use crate::types::{Database, IdentifierMeta};

pub struct JoinClause<'a, DB: Database> {
    pub(crate) items: Vec<Join<'a, DB>>,
}

impl<'a, DB: Database> JoinClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn add(&mut self, join: Join<'a, DB>) {
        self.items.push(join);
    }

    pub(crate) fn tag_latest(&mut self, tag: TypeId) {
        if let Some(item) = self.items.last_mut() {
            item.tag = Some(tag);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

pub struct Join<'a, DB: Database> {
    pub typ: JoinType,
    pub target_table: &'static str,
    pub tag: Option<TypeId>,
    pub constraint: JoinConstraint<'a, DB>,
}

pub struct JoinConstraint<'a, DB: Database> {
    pub(crate) kind: JoinConstraintKind<'a, DB>,
}

pub(crate) enum JoinConstraintKind<'a, DB: Database> {
    On(Condition<'a, DB, ()>),
    Using(Vec<IdentifierMeta>),
    None, // For Cross join, no conditions required
}

impl<'a, DB: Database> JoinConstraint<'a, DB> {
    #[inline(always)]
    pub fn on(cond: Condition<'a, DB, ()>) -> Self {
        Self { kind: JoinConstraintKind::On(cond) }
    }

    #[inline(always)]
    pub fn using(cols: Vec<IdentifierMeta>) -> Self {
        Self { kind: JoinConstraintKind::Using(cols) }
    }

    #[inline(always)]
    pub fn none() -> Self {
        Self { kind: JoinConstraintKind::None }
    }
}

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a valid JOIN constraint",
    label = "invalid JOIN constraint",
    note = "for `ON` conditions, combine boolean conditions with `&` or `|` (not a tuple).\nfor `USING (...)` joins, pass a column or a tuple of columns `(col1, col2)`."
)]
pub trait IntoJoinConstraint<'a, DB: Database, X> {
    fn into_constraint(self) -> JoinConstraint<'a, DB>;
}

// 1. ON branch: Accepts only a single Condition, statically excluding (cond1, cond2) or (cond, col)
impl<'a, DB: Database, X> IntoJoinConstraint<'a, DB, X> for Condition<'a, DB, X> {
    #[inline(always)]
    fn into_constraint(self) -> JoinConstraint<'a, DB> {
        JoinConstraint::on(self.erase_x())
    }
}

// 2. USING single branch: Implementing `IntoArg<IdentifierMeta, ArgAst>` is sufficient
impl<'a, DB: Database, X, T> IntoJoinConstraint<'a, DB, X> for T
    where
        T: IntoArg<IdentifierMeta, ArgAst> {
    #[inline(always)]
    fn into_constraint(self) -> JoinConstraint<'a, DB> {
        JoinConstraint::using(self.into_args())
    }
}

// 3. USING Multiple columns tuple branch
macro_rules! impl_into_join_constraint_tuple {
    ( $($T:ident),+ ) => {
        #[diagnostic::do_not_recommend]
        impl<'a, DB: Database, X, $($T),+> IntoJoinConstraint<'a, DB, X> for ($($T,)+)
            where
                $($T: IntoArg<IdentifierMeta, ArgAst> ),+ {
            #[inline(always)]
            fn into_constraint(self) -> JoinConstraint<'a, DB> {
                JoinConstraint::using(self.into_args())
            }
        }
    };
}

#[rustfmt::skip]
mod macros {
    use super::*;
    impl_into_join_constraint_tuple!(T1);
    impl_into_join_constraint_tuple!(T1, T2);
    impl_into_join_constraint_tuple!(T1, T2, T3);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32);
}
