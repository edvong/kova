use crate::dsl::{EraseX, IntoIdentifier, Operand};
use crate::types::Database;
use crate::types::IdentifierMeta;
use std::marker::PhantomData;

pub struct ProjectionClause<'a, DB: Database> {
    items: Vec<Projection<'a, DB, ()>>,
}

impl<'a, DB: Database> ProjectionClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn set(&mut self, args: Vec<Projection<'a, DB, ()>>) {
        self.items = args;
    }
}

pub struct Projection<'a, DB: Database, X> {
    pub(crate) operand: Operand<'a, DB>,
    pub(crate) alias: Option<IdentifierMeta>,
    _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> Projection<'a, DB, X> {
    pub fn new(operand: Operand<'a, DB>) -> Self {
        Self { operand, alias: None, _marker: PhantomData }
    }

    pub fn alias<A, T>(mut self, alias: A) -> Self
        where
            A: IntoIdentifier<T> {
        self.alias = Some(alias.into_identifier().meta);
        self
    }
}

impl<'a, DB: Database, X> EraseX for Projection<'a, DB, X> {
    type Erased = Projection<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Projection { operand: self.operand, alias: self.alias, _marker: PhantomData }
    }
}

pub trait IntoProjection<'a, DB: Database, X> {
    fn into_projection(self) -> Projection<'a, DB, X>;
}
