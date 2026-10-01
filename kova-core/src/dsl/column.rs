#![allow(clippy::wrong_self_convention)]

use crate::dsl::arg::SingleArgMarker;
use crate::dsl::component::WithComponent;
use crate::dsl::identifier::{Identifier, IntoIdentifier};
use crate::dsl::operand::IntoOperand;
use crate::dsl::{Assignment, BuildOrdering, Direction, Ordering, Projection, ProjectionExpr, StandardAssignment, StandardOperand};
use crate::dsl::{
    IntoArg, OpAst, OpNode, Operand, OrderOperand,
};
use crate::type_id::type_id;
use crate::types::{ColumnMeta, Database, Extension, IdentifierMeta, SyncMeta};
use std::marker::PhantomData;
use std::ops::Deref;

pub type ExtColumn<T, DB> = <<DB as Database>::Extension<'static> as Extension<'static>>::Column<T>;

pub struct Column<T, DB: Database> {
    pub(crate) meta: ColumnMeta,
    inner: ExtColumn<T, DB>,
}

impl<DB: Database, T> Copy for Column<T, DB> {}
impl<DB: Database, T> Clone for Column<T, DB> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T, DB: Database> Deref for Column<T, DB> {
    type Target = ExtColumn<T, DB>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T, DB: Database> IntoArg<Operand<'a, DB>> for Column<T, DB> {
    #[inline(always)]
    fn into_arg(self) -> Operand<'a, DB> {
        Operand::from_identifier(self.meta.ident)
    }
}

impl<'a, T, DB: Database> IntoIdentifier<T> for Column<T, DB> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta.ident)
    }
}

impl<T, DB: Database> IntoIdentifier<T> for &Column<T, DB> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta.ident)
    }
}

impl<T, DB: Database> SingleArgMarker for Column<T, DB> {}

impl<T, DB: Database> IntoArg<IdentifierMeta> for Column<T, DB> {
    #[inline(always)]
    fn into_arg(self) -> IdentifierMeta {
        self.meta.ident
    }
}

impl<T, DB: Database> IntoArg<IdentifierMeta> for &Column<T, DB> {
    #[inline(always)]
    fn into_arg(self) -> IdentifierMeta {
        self.meta.ident
    }
}

impl<'a, T, DB: Database, X> BuildOrdering<'a, DB, X> for &Column<T, DB> {
    #[inline(always)]
    fn build_ordering(self, direction: Option<Direction>) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Column(self.meta.ident)),
            direction,
            nulls: None,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, DB: Database, X> IntoOperand<'a, T, DB, OpAst, X> for Column<T, DB> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_identifier(self.into_identifier().meta)
    }
}

impl<'a, T, DB: Database> OpNode for Column<T, DB> {
    type Input = ();
    type Output = T;
}

impl<'a, T, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Column<T, DB> {
    fn into_arg(self) -> Projection<'a, DB, X> {
        Projection::new(ProjectionExpr::Column(self.into_identifier().meta), None)
    }
}

impl<'a, T, DB: Database> SyncMeta<ColumnMeta> for Column<T, DB> {
    fn sync(&mut self, meta: ColumnMeta) {
        self.meta = meta;
        self.inner.sync(meta);
    }
}

/// Unconstrained impl blocks, specialized for `const fn` constructors
impl<T, DB: Database> Column<T, DB> {
    pub const fn new(meta: ColumnMeta, ext: ExtColumn<T, DB>) -> Self {
        Self { meta, inner: ext }
    }

    pub fn meta(self) -> ColumnMeta {
        self.meta
    }

    pub fn identifier(self) -> IdentifierMeta {
        self.meta().ident
    }

    // Defined at the outermost level
    #[inline(always)]
    pub fn tag<Marker: 'static>(mut self) -> Self {
        self.meta.ident.with_tag(type_id::<Marker>());
        self.inner.sync(self.meta);
        self
    }

    pub fn set<'a, V, K, X>(self, v: V) -> Assignment<'a, DB, X>
    where
        V: IntoOperand<'a, T, DB, K, X> {
        Assignment::from_std(StandardAssignment::Value(self.meta.ident, v.into_operand()))
    }

    pub fn increment<'a, V, U, K, X>(self, v: V) -> Assignment<'a, DB, X>
    where
        V: IntoOperand<'a, U, DB, K, X> {
        Assignment::from_std(StandardAssignment::Increment(self.meta.ident, v.into_operand()))
    }

    pub fn decrement<'a, V, U, K, X>(self, v: V) -> Assignment<'a, DB, X>
    where
        V: IntoOperand<'a, U, DB, K, X> {
        Assignment::from_std(StandardAssignment::Decrement(self.meta.ident, v.into_operand()))
    }

}