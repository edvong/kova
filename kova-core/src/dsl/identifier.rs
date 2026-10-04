#![allow(clippy::wrong_self_convention)]

use crate::dsl::{
    IntoOperand, IntoOrdering, OpAst, OpNode, Operand, Ordering, SingleArgMarker,
};
use crate::type_id::type_id;
use crate::types::{Database, IdentifierMeta};
use std::marker::PhantomData;

pub trait IntoIdentifier<T> {
    fn into_identifier(self) -> Identifier<T>;
}

impl<T> IntoIdentifier<T> for Identifier<T> {
    fn into_identifier(self) -> Identifier<T> {
        self
    }
}

impl<T> IntoIdentifier<T> for &Identifier<T> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier { meta: self.meta, _marker: PhantomData }
    }
}

pub struct Identifier<T> {
    pub meta: IdentifierMeta,
    _marker: PhantomData<T>,
}

// impl<'a, T, DB: Database> IntoArg<Operand<'a, DB>> for Identifier<T> {
//     #[inline(always)]
//     fn into_arg(self) -> Operand<'a, DB> {
//         Operand::from_identifier(self.meta)
//     }
// }

impl<'a, T, DB: Database, X> IntoOperand<'a, T, DB, OpAst, X> for Identifier<T> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_column(self.into_identifier().meta)
    }
}

impl<'a, DB: Database, T, X> IntoOrdering<'a, DB, X> for Identifier<T> {
    fn into_ordering(self) -> Ordering<'a, DB, X> {
        Ordering::from_operand(IntoOperand::<'a, T, DB, OpAst, X>::into_operand(self))
    }
}

impl<T> OpNode for Identifier<T> {
    type Input = ();
    type Output = T;
}

impl<T> SingleArgMarker for Identifier<T> {}

impl<T> Identifier<T> {
    pub const fn from(meta: IdentifierMeta) -> Self {
        Self { meta, _marker: PhantomData }
    }

    pub const fn new(source: &'static str, name: &'static str) -> Self {
        Self { meta: IdentifierMeta::new(source, name), _marker: PhantomData }
    }

    // For CTE and View
    #[inline(always)]
    pub fn tag<Marker: 'static>(mut self) -> Self {
        self.meta = self.meta.with_tag(type_id::<Marker>());
        self
    }
}

/*
impl<T> Identifier<T> {
    pub fn eq<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
                where
                    V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().eq(v)
    }

    pub fn ne<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
                where
                    V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().ne(v)
    }

    pub fn gt<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
                where
                    V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().gt(v)
    }

    pub fn ge<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
                where
                    V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().ge(v)
    }

    pub fn lt<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
                where
                    V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().lt(v)
    }

    pub fn le<'a, DB: Database, V, K, X>(self, v: V) -> Condition<'a, DB, X>
                where
                    V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().le(v)
    }

    pub fn is_null<'a, DB: Database, X>(self) -> Condition<'a, DB, X> {
        IntoComparator::<'a, T, DB>::into_comparator(self).is_null()
    }

    pub fn is_not_null<'a, DB: Database, X>(self) -> Condition<'a, DB, X> {
        IntoComparator::<'a, T, DB>::into_comparator(self).is_not_null()
    }

    pub fn between<'a, DB: Database, L, U, K, X>(self, lower: L, upper: U) -> Condition<'a, DB, X>
                where
                    L: IntoOperand<'a, T, DB, K, X>,
                    U: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().between(lower, upper)
    }

    pub fn not_between<'a, DB: Database, L, U, K, X>(
        self, lower: L, upper: U,
    ) -> Condition<'a, DB, X>
                where
                    L: IntoOperand<'a, T, DB, K, X>,
                    U: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().not_between(lower, upper)
    }

    pub fn in_list<'a, DB: Database, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
                where
                    I: IntoIterator<Item = V>,
                    V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().in_list(values)
    }

    pub fn not_in_list<'a, DB: Database, I, V, K, X>(self, values: I) -> Condition<'a, DB, X>
                where
                    I: IntoIterator<Item = V>,
                    V: IntoOperand<'a, T, DB, K, X> {
        self.into_comparator().not_in_list(values)
    }

    pub fn asc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Asc))
    }

    pub fn desc<'a, DB: Database, X>(self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Desc))
    }
}
*/
