use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    BuildOrdering, BuildProjection, Direction, IntoArg, IntoIdentifier, OrderOperand, Ordering,
    Projection, ProjectionExpr, SingleArgMarker, StandardOperand, WithComponent,
};
use std::marker::PhantomData;

// Relevance Scoring
pub struct Bm25<'a, X> {
    pub(crate) table: &'static str,
    pub(crate) _marker: PhantomData<&'a X>,
}

impl<'a, X> Bm25<'a, X> {
    #[inline(always)]
    fn into_func(self) -> Function<'a, Sqlite> {
        Function::from_ext(ExtFunction::Bm25(self.table))
    }

    pub fn asc(self) -> Ordering<'a, Sqlite, X> {
        self.build_ordering(Some(Direction::Asc))
    }

    pub fn desc(self) -> Ordering<'a, Sqlite, X> {
        self.build_ordering(Some(Direction::Desc))
    }

    pub fn alias<I: IntoIdentifier<A>, A>(self, alias: I) -> Projection<'a, Sqlite, X> {
        self.build_projection(alias)
    }
}

impl<'a, X> BuildOrdering<'a, Sqlite, X> for Bm25<'a, X> {
    #[inline(always)]
    fn build_ordering(self, direction: Option<Direction>) -> Ordering<'a, Sqlite, X> {
        Ordering::new(
            OrderOperand::from_std(StandardOperand::Function(self.into_func())),
            direction,
            None,
        )
    }
}

impl<'a, X> BuildProjection<'a, Sqlite, X> for Bm25<'a, X> {
    #[inline(always)]
    fn build_projection<A, U>(self, alias: A) -> Projection<'a, Sqlite, X>
        where
            A: IntoIdentifier<U> {
        Projection::new(
            ProjectionExpr::Function(self.into_func()),
            Some(alias.into_identifier().meta),
        )
    }
}

impl<'a, X> SingleArgMarker for Bm25<'a, X> {}

impl<'a, X> IntoArg<Projection<'a, Sqlite, X>> for Bm25<'a, X> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        Projection::new(ProjectionExpr::Function(self.into_func()), None)
    }
}
