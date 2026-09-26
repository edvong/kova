use crate::dsl::{Column, Condition, EraseX, SingleArgMarker};
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

    pub(crate) fn tag_latest(&mut self, tag: usize) {
        if let Some(item) = self.items.last_mut() {
            item.tag = Some(tag);
        }
    }
}

pub struct Join<'a, DB: Database> {
    pub typ: JoinType,
    pub target_table: &'static str,
    pub tag: Option<usize>,
    pub constraint: JoinConstraint<'a, DB>, // pub on: Condition<'a, DB, ()>,
}

pub enum JoinConstraint<'a, DB: Database> {
    On(Condition<'a, DB, ()>),
    Using(Vec<IdentifierMeta>),
    None, // For Cross join, no conditions required
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

pub trait IntoJoinConstraint<'a, DB: Database, X> {
    fn into_constraint(self) -> JoinConstraint<'a, DB>;
}

impl<'a, DB: Database, X> IntoJoinConstraint<'a, DB, X> for Condition<'a, DB, X> {
    #[inline(always)]
    fn into_constraint(self) -> JoinConstraint<'a, DB> {
        JoinConstraint::On(self.erase_x())
    }
}

impl<'a, DB: Database, X> SingleArgMarker for Condition<'a, DB, X> {}

impl<'a, T, DB: Database, X> IntoJoinConstraint<'a, DB, X> for Column<'a, T, DB> {
    #[inline(always)]
    fn into_constraint(self) -> JoinConstraint<'a, DB> {
        JoinConstraint::Using(vec![self.meta])
    }
}

// impl<'a, T, DB: Database> IntoJoinConstraint<'a, DB> for &Column<'a, T, DB> {
//     #[inline(always)]
//     fn into_constraint(self) -> JoinConstraint<'a, DB> {
//         JoinConstraint::Using(vec![self.meta])
//     }
// }

macro_rules! impl_into_join_constraint_tuple {
    ( $($T:ident),+ ) => {
        #[diagnostic::do_not_recommend]
        impl<'a, DB: Database, X, $($T),+> IntoJoinConstraint<'a, DB, X> for ($($T,)+)
        where
            $( $T: $crate::dsl::IntoArg<IdentifierMeta> ),+
        {
            fn into_constraint(self) -> JoinConstraint<'a, DB> {
                #[allow(non_snake_case)]
                let ( $($T,)+ ) = self;
                JoinConstraint::Using(vec![ $( $T.into_arg() ),+ ])
            }
        }
    };
}

impl_into_join_constraint_tuple!(T1);
impl_into_join_constraint_tuple!(T1, T2);
impl_into_join_constraint_tuple!(T1, T2, T3);
impl_into_join_constraint_tuple!(T1, T2, T3, T4);
impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5);
impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6);
impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_into_join_constraint_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
