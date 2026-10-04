#![allow(clippy::wrong_self_convention)]

use crate::dsl::function::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    Comparator, Condition, IntoArg, IntoComparator, IntoIdentifier, IntoOperand, IntoProjection,
    OpAst, Operand, Projection, SingleArgMarker,
};
use std::marker::PhantomData;

pub struct JsonExtract<'a, X> {
    pub(crate) base: Operand<'a, Sqlite>,
    pub(crate) path: Operand<'a, Sqlite>,
    pub(crate) _marker: PhantomData<X>,
}

impl<'a, X> SingleArgMarker for JsonExtract<'a, X> {}

impl<'a, T, X> IntoOperand<'a, T, Sqlite, OpAst, X> for JsonExtract<'a, X> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(ExtFunction::JsonExtract(
            Box::new(self.base),
            Box::new(self.path),
        )))
    }
}

impl<'a, X> IntoProjection<'a, Sqlite, X> for JsonExtract<'a, X> {
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(IntoOperand::<(), Sqlite, OpAst, X>::into_operand(self))
    }
}

impl<'a, T, X> IntoComparator<'a, T, Sqlite> for JsonExtract<'a, X> {
    #[inline(always)]
    fn into_comparator(self) -> Comparator<'a, T, Sqlite> {
        let func = ExtFunction::JsonExtract(Box::new(self.base), Box::new(self.path));
        Comparator::new(Operand::from_function(Function::from(func)))
    }
}

impl<'a, X> IntoArg<Projection<'a, Sqlite, X>> for JsonExtract<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}

// Slapping the alias method onto JsonExtract. Keeping it in its own impl block to keep things clean.
impl<'a, X> JsonExtract<'a, X> {
    pub fn new(base: Operand<'a, Sqlite>, path: Operand<'a, Sqlite>) -> JsonExtract<'a, X> {
        Self { base, path, _marker: PhantomData }
    }

    pub fn alias<I: IntoIdentifier<A>, A>(self, alias: I) -> Projection<'a, Sqlite, X> {
        self.into_projection().alias(alias)
    }
}

impl<'a, X> JsonExtract<'a, X> {
    pub fn eq<Target, V, K>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().eq(v)
    }

    pub fn ne<Target, V, K>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().ne(v)
    }

    pub fn gt<Target, V, K>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().gt(v)
    }

    pub fn ge<Target, V, K>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().ge(v)
    }

    pub fn lt<Target, V, K>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().lt(v)
    }

    pub fn le<Target, V, K>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().le(v)
    }

    pub fn like<Target, V, K>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().like(v)
    }

    pub fn not_like<Target, V, K>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().not_like(v)
    }

    pub fn in_list<Target, I, V, K>(self, values: I) -> Condition<'a, Sqlite, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().in_list(values)
    }

    pub fn not_in_list<Target, I, V, K>(self, values: I) -> Condition<'a, Sqlite, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, Sqlite, K, X> {
        self.into_comparator().not_in_list(values)
    }

    // Trick: Use `()` as a virtual Target here to seamlessly bypass strict type checking for null checks.
    pub fn is_null(self) -> Condition<'a, Sqlite, X> {
        IntoComparator::<'a, (), Sqlite>::into_comparator(self).is_null()
    }

    pub fn is_not_null(self) -> Condition<'a, Sqlite, X> {
        IntoComparator::<'a, (), Sqlite>::into_comparator(self).is_not_null()
    }
}
