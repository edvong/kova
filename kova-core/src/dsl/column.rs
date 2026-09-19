use crate::dsl::arg::SingleArgMarker;
use crate::dsl::comparator::AsComparator;
use crate::dsl::component::WithComponent;
use crate::dsl::condition::Condition;
use crate::dsl::identifier::{Identifier, IntoIdentifier};
use crate::dsl::operand::IntoOperand;
use crate::dsl::{
    Assignment, Comparator, IntoArg, OpAst, Operand, OrderOperand, StandardAssignment,
};
use crate::dsl::{BuildOrdering, Direction, Ordering, Projection, ProjectionExpr, StandardOperand};
use crate::types::{Database, Extension, IdentifierMeta};
use std::marker::PhantomData;
use std::ops::Deref;

pub type ExtColumn<'a, T, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Column<'a, T>;

pub struct Column<'a, T: 'a, DB: Database> {
    pub(crate) meta: IdentifierMeta,
    inner: ExtColumn<'a, T, DB>,
}

impl<'a, T, DB: Database> Deref for Column<'a, T, DB> {
    type Target = ExtColumn<'a, T, DB>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T, DB: Database> IntoArg<Operand<'a, DB>> for Column<'a, T, DB> {
    #[inline(always)]
    fn into_arg(self) -> Operand<'a, DB> {
        Operand::from_identifier(self.meta)
    }
}

// impl<'a, T, DB: Database> IntoArg<Operand<'a, DB>> for &Column<'a, T, DB> {
//     #[inline(always)]
//     fn into_arg(self) -> Operand<'a, DB> {
//         Operand::from_identifier(self.meta)
//     }
// }

impl<'a, T, DB: Database> IntoIdentifier<T> for Column<'a, T, DB> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta)
    }
}

impl<'a, T, DB: Database> IntoIdentifier<T> for &Column<'a, T, DB> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta)
    }
}

impl<'a, T, DB: Database, X> BuildOrdering<'a, DB, X> for &Column<'a, T, DB> {
    #[inline(always)]
    fn build_ordering(self, direction: Option<Direction>) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Column(self.meta)),
            direction,
            nulls: None,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, DB: Database, X> IntoOperand<'a, T, DB, OpAst, X> for Column<'a, T, DB> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_identifier(self.into_identifier().meta)
    }
}

impl<'a, T, DB: Database> AsComparator<'a, T, DB> for Column<'a, T, DB> {
    #[inline(always)]
    fn as_comparator(&self) -> Comparator<'a, T, DB> {
        Comparator::new(Operand::from_identifier(self.into_identifier().meta))
    }
}

impl<'a, T, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Column<'a, T, DB> {
    fn into_arg(self) -> Projection<'a, DB, X> {
        Projection::new(ProjectionExpr::Column(self.into_identifier().meta), None)
    }
}

impl<'a, T, DB: Database> SingleArgMarker for Column<'a, T, DB> {}

impl<'a, T, DB: Database> Column<'a, T, DB> {
    /// The type of alias T must be consistent with the current column
    pub fn alias<X>(self, alias: Column<'a, T, DB>) -> Projection<'a, DB, X> {
        Projection::new(
            ProjectionExpr::Column(self.into_identifier().meta),
            Some(alias.into_identifier().meta),
        )
    }
}

/// Unconstrained impl blocks, specialized for `const fn` constructors
impl<'a, T, DB: Database> Column<'a, T, DB> {
    pub const fn new(meta: IdentifierMeta, ext: ExtColumn<'a, T, DB>) -> Self {
        Self { meta, inner: ext }
    }
    pub fn meta(&self) -> IdentifierMeta {
        self.meta
    }
}

impl<'a, T, DB: Database> Column<'a, T, DB> {
    pub fn eq<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().eq(v)
    }

    pub fn ne<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().ne(v)
    }

    pub fn gt<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().gt(v)
    }

    pub fn ge<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().ge(v)
    }

    pub fn lt<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().lt(v)
    }

    pub fn le<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().le(v)
    }

    pub fn is_null<X>(&self) -> Condition<'a, DB, X> {
        self.as_comparator().is_null()
    }

    pub fn is_not_null<X>(&self) -> Condition<'a, DB, X> {
        self.as_comparator().is_not_null()
    }

    pub fn between<L, U, K, X>(&self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, T, DB, K, X>,
            U: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().between(lower, upper)
    }

    pub fn not_between<L, U, K, X>(&self, lower: L, upper: U) -> Condition<'a, DB, X>
        where
            L: IntoOperand<'a, T, DB, K, X>,
            U: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().not_between(lower, upper)
    }

    pub fn in_list<I, V, K, X>(&self, values: I) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().in_list(values)
    }

    pub fn not_in<I, V, K, X>(&self, values: I) -> Condition<'a, DB, X>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().not_in_list(values)
    }

    pub fn like<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().like(v)
    }

    pub fn not_like<V, K, X>(&self, v: V) -> Condition<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        self.as_comparator().not_like(v)
    }

    pub fn asc<X>(&self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Asc))
    }

    pub fn desc<X>(&self) -> Ordering<'a, DB, X> {
        self.build_ordering(Some(Direction::Desc))
    }

    pub fn set<V, K, X>(&self, v: V) -> Assignment<'a, DB, X>
        where
            V: IntoOperand<'a, T, DB, K, X> {
        Assignment::from_std(StandardAssignment::Value(self.meta, v.into_operand()))
    }

    pub fn increment<V, U, K, X>(&self, v: V) -> Assignment<'a, DB, X>
        where
            V: IntoOperand<'a, U, DB, K, X> {
        Assignment::from_std(StandardAssignment::Increment(self.meta, v.into_operand()))
    }

    pub fn decrement<V, U, K, X>(&self, v: V) -> Assignment<'a, DB, X>
        where
            V: IntoOperand<'a, U, DB, K, X> {
        Assignment::from_std(StandardAssignment::Decrement(self.meta, v.into_operand()))
    }
}
