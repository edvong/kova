#![allow(clippy::wrong_self_convention)]

use crate::dsl::{Comparator, Condition, Direction, GroupBy, Identifier, IntoArg, IntoGroupBy, IntoIdentifier, IntoOperand, IntoOrdering, IntoProjection, OpAst, OpNode, Operand, Ordering, Projection, SingleArgMarker};
use crate::type_id;
use crate::types::{Database, Extension, IdentifierMeta, SyncMeta};
use std::marker::PhantomData;
use std::ops::Deref;

pub type ExtVirtualColumn<DB, T> =
    <<DB as Database>::Extension<'static> as Extension<'static>>::VirtualColumn<T>;

pub struct Strict;
pub struct Dynamic;

// For View, SRF virtual tables, CTE fields,
// and other columns without specific physical type constraints.
pub struct VirtualColumn<DB: Database, T = (), S = Strict> {
    pub meta: IdentifierMeta,
    ext: ExtVirtualColumn<DB, T>,
    _marker: PhantomData<S>,
}

impl<DB: Database, T, S> Copy for VirtualColumn<DB, T, S> {}
impl<DB: Database, T, S> Clone for VirtualColumn<DB, T, S> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<DB: Database, T, S> Deref for VirtualColumn<DB, T, S> {
    type Target = ExtVirtualColumn<DB, T>;

    fn deref(&self) -> &Self::Target {
        &self.ext
    }
}

impl<DB: Database, T, S> SyncMeta<IdentifierMeta> for VirtualColumn<DB, T, S> {
    fn sync(&mut self, meta: IdentifierMeta) {
        self.meta = meta;
        self.ext.sync(meta);
    }
}

impl<DB: Database, T, S> VirtualColumn<DB, T, S> {
    pub const fn new(meta: IdentifierMeta, ext: ExtVirtualColumn<DB, T>) -> Self {
        Self { meta, ext, _marker: PhantomData }
    }

    pub fn tag<Marker: 'static>(mut self) -> Self {
        self.sync(self.meta.with_tag(type_id::<Marker>()));
        self
    }

    pub fn alias<'a, I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, DB, X> {
        self.into_projection().alias(alias)
    }

    pub fn eq<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).eq(v)
    }

    pub fn ne<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).ne(v)
    }

    pub fn gt<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).gt(v)
    }

    pub fn ge<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).ge(v)
    }

    pub fn lt<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).lt(v)
    }

    pub fn le<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).le(v)
    }

    pub fn is_null<'a, X>(self) -> Condition<'a, DB, X> {
        Comparator::<'a, (), DB>::new(Operand::from_column(self.meta)).is_null()
    }

    pub fn is_not_null<'a, X>(self) -> Condition<'a, DB, X> {
        Comparator::<'a, (), DB>::new(Operand::from_column(self.meta)).is_not_null()
    }

    pub fn between<'a, Target, L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            L: IntoOperand<'a, Target, DB, K, X>,
            U: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).between(lower, upper)
    }

    pub fn not_between<'a, Target, L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            L: IntoOperand<'a, Target, DB, K, X>,
            U: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).not_between(lower, upper)
    }

    pub fn in_list<'a, Target, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).in_list(values)
    }

    pub fn not_in_list<'a, Target, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).not_in_list(values)
    }

    pub fn in_select<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).in_select(v)
    }

    pub fn not_in_select<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).not_in_select(v)
    }

    pub fn like<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).like(v)
    }

    pub fn not_like<'a, Target, V, K, X>(self, v: V) -> Condition<'a, DB, X>
        where
            Self: IntoOperand<'a, Target, DB, OpAst, X>,
            V: IntoOperand<'a, Target, DB, K, X> {
        Comparator::<'a, Target, DB>::new(self.into_operand()).not_like(v)
    }

    // pub fn asc<'a, X>(self) -> Ordering<'a, DB, X> {
    //     Ordering::from_operand(IntoOperand::<'a, T, DB, OpAst, X>::into_operand(self))
    //         .direction(Direction::Asc)
    // }
    //
    // pub fn desc<'a, X>(self) -> Ordering<'a, DB, X> {
    //     Ordering::from_operand(IntoOperand::<'a, T, DB, OpAst, X>::into_operand(self))
    //         .direction(Direction::Desc)
    // }
}

impl<DB: Database, T> VirtualColumn<DB, T, Strict> {
    pub fn asc<'a, X>(self) -> Ordering<'a, DB, X> {
        self.into_ordering().direction(Direction::Asc)
    }

    pub fn desc<'a, X>(self) -> Ordering<'a, DB, X> {
        self.into_ordering().direction(Direction::Desc)
    }
}

impl<DB: Database, T> VirtualColumn<DB, T, Dynamic> {
    pub fn asc<'a, X>(self) -> Ordering<'a, DB, X> {
        self.into_ordering().direction(Direction::Asc)
    }

    pub fn desc<'a, X>(self) -> Ordering<'a, DB, X> {
        self.into_ordering().direction(Direction::Desc)
    }
}

impl<DB: Database, T> OpNode for VirtualColumn<DB, T> {
    type Input = ();
    type Output = T;
}

impl<DB: Database, T, S> SingleArgMarker for VirtualColumn<DB, T, S> {}

impl<DB: Database, T, S> IntoIdentifier<T> for VirtualColumn<DB, T, S> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta)
    }
}

impl<DB: Database, T, S> IntoIdentifier<T> for &VirtualColumn<DB, T, S> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta)
    }
}

// impl<DB: Database, T, S> IntoArg<IdentifierMeta> for VirtualColumn<DB, T, S> {
//     #[inline(always)]
//     fn into_arg(self) -> IdentifierMeta {
//         self.meta
//     }
// }
//
// impl<DB: Database, T, S> IntoArg<IdentifierMeta> for &VirtualColumn<DB, T, S> {
//     #[inline(always)]
//     fn into_arg(self) -> IdentifierMeta {
//         self.meta
//     }
// }

impl<'a, DB: Database, T, X> IntoOperand<'a, T, DB, OpAst, X> for VirtualColumn<DB, T, Strict> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_column(self.meta)
    }
}

impl<'a, DB: Database, T, Target, X> IntoOperand<'a, Target, DB, OpAst, X>
    for VirtualColumn<DB, T, Dynamic> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_column(self.meta)
    }
}

impl<'a, DB: Database, T, S, X> IntoOrdering<'a, DB, X> for VirtualColumn<DB, T, S> {
    #[inline(always)]
    fn into_ordering(self) -> Ordering<'a, DB, X> {
        Ordering::from_operand(Operand::from_column(self.meta))
        // Ordering::from_operand(IntoOperand::<'a, T, DB, OpAst, X>::into_operand(self))
    }
}

impl<'a, DB: Database, T, S, X> IntoProjection<'a, DB, X> for VirtualColumn<DB, T, S> {
    #[inline(always)]
    fn into_projection(self) -> Projection<'a, DB, X> {
        Projection::new(Operand::from_column(self.meta))
    }
}

impl<'a, DB: Database, T, S, X> IntoGroupBy<'a, DB, X> for VirtualColumn<DB, T, S> {
    #[inline(always)]
    fn into_group_by(self) -> GroupBy<'a, DB, X> {
        GroupBy::new(Operand::from_column(self.meta))
    }
}

impl<'a, DB: Database, T, S, X> IntoArg<Projection<'a, DB, X>> for VirtualColumn<DB, T, S> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, DB, X> {
        self.into_projection()
    }
}

impl<'a, DB: Database, T, S, X> IntoArg<GroupBy<'a, DB, X>> for VirtualColumn<DB, T, S> {
    #[inline(always)]
    fn into_arg(self) -> GroupBy<'a, DB, X> {
        self.into_group_by()
    }
}
