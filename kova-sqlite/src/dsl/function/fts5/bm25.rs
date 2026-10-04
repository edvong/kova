use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    Direction, IntoArg, IntoIdentifier, IntoOperand, IntoOrdering, IntoProjection, OpAst, Operand,
    Ordering, Projection, SingleArgMarker,
};
use std::marker::PhantomData;

// Relevance Scoring
pub struct Bm25<'a, X> {
    table: &'static str,
    weights: Vec<f64>,
    _marker: PhantomData<&'a X>,
}

impl<'a, X> Bm25<'a, X> {
    pub(crate) fn new(table: &'static str) -> Self {
        Self {table, weights: Vec::new(), _marker: PhantomData }
    }

    pub fn weights(mut self, weights: impl Into<Vec<f64>>) -> Self {
        self.weights = weights.into();
        self
    }

    pub fn asc(self) -> Ordering<'a, Sqlite, X> {
        self.into_ordering().direction(Direction::Asc)
    }

    pub fn desc(self) -> Ordering<'a, Sqlite, X> {
        self.into_ordering().direction(Direction::Desc)
    }

    pub fn alias<I: IntoIdentifier<A>, A>(self, alias: I) -> Projection<'a, Sqlite, X> {
        self.into_projection().alias(alias)
    }

    #[inline(always)]
    fn into_func(self) -> Function<'a, Sqlite> {
        Function::from(ExtFunction::Bm25(self.table, self.weights))
    }
}

impl<'a, X> SingleArgMarker for Bm25<'a, X> {}

impl<'a, X> IntoOperand<'a, (), Sqlite, OpAst, X> for Bm25<'a, X> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(self.into_func())
    }
}

impl<'a, X> IntoOrdering<'a, Sqlite, X> for Bm25<'a, X> {
    fn into_ordering(self) -> Ordering<'a, Sqlite, X> {
        Ordering::from_operand(self.into_operand())
    }
}

impl<'a, X> IntoProjection<'a, Sqlite, X> for Bm25<'a, X> {
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(self.into_operand())
    }
}

impl<'a, X> IntoArg<Projection<'a, Sqlite, X>> for Bm25<'a, X> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}
