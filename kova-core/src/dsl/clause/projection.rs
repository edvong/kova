use crate::dsl::arg::SingleArgMarker;
use crate::dsl::function::Function;
use crate::dsl::identifier::IdentifierMeta;
use crate::dsl::statement::SelectStatement;
use crate::dsl::{EraseX, IntoIdentifier};
use crate::types::Database;
use std::marker::PhantomData;

pub struct ProjectionClause<'a, DB: Database> {
    items: Vec<Projection<'a, DB, ()>>,
}

impl<'a, DB: Database> ProjectionClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub(crate) fn from(items: Vec<Projection<'a, DB, ()>>) -> Self {
        Self { items }
    }
    pub fn set(&mut self, args: Vec<Projection<'a, DB, ()>>) {
        self.items = args;
    }
}

pub struct Projection<'a, DB: Database, X> {
    pub(crate) expr: ProjectionExpr<'a, DB>,
    pub(crate) alias: Option<IdentifierMeta>,
    _marker: PhantomData<X>,
}

pub trait BuildProjection<'a, DB: Database, X> {
    fn build_projection<A, T>(self, alias: A) -> Projection<'a, DB, X>
        where
            A: IntoIdentifier<T>;
}

impl<'a, DB: Database, X> EraseX for Projection<'a, DB, X> {
    type Erased = Projection<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Projection { expr: self.expr, alias: self.alias, _marker: PhantomData }
    }
}

impl<'a, DB: Database, X> Projection<'a, DB, X> {
    pub fn new(expr: ProjectionExpr<'a, DB>, alias: Option<IdentifierMeta>) -> Self {
        Self { expr, alias, _marker: PhantomData }
    }

    pub fn alias<A, T>(&mut self, alias: A)
        where
            A: IntoIdentifier<T> {
        self.alias = Some(alias.into_identifier().meta);
    }
}

pub enum ProjectionExpr<'a, DB: Database> {
    Identifier(IdentifierMeta),
    Function(Function<'a, DB>),
    Subselect(Box<SelectStatement<'a, DB>>),
}

impl<'a, DB: Database, X> SingleArgMarker for Projection<'a, DB, X> {}

// impl<'a, T, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Column<'a, T, DB> {
//     fn into_arg(self) -> Projection<'a, DB, X> {
//         Projection::new(
//             ProjectionExpr::Column(self.meta),
//             None,
//         )
//     }
// }

// impl<'a, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Function<'a, DB> {
//     fn into_arg(self) -> Projection<'a, DB, X> {
//         Projection::new (
//             ProjectionExpr::Function(self),
//             None,
//         )
//     }
// }
