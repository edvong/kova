use crate::dsl::arg::SingleArgMarker;
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
    pub(crate) fn from(items: Vec<Projection<'a, DB, ()>>) -> Self {
        Self { items }
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

impl<'a, DB: Database, X> SingleArgMarker for Projection<'a, DB, X> {}

impl<'a, DB: Database, X> EraseX for Projection<'a, DB, X> {
    type Erased = Projection<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Projection { operand: self.operand, alias: self.alias, _marker: PhantomData }
    }
}

pub trait IntoProjection<'a, DB: Database, X> {
    fn into_projection(self) -> Projection<'a, DB, X>;
}


// impl<'a, DB: Database, X> Projection<'a, DB, X> {
//     pub fn transform_expr<F>(mut self, f: F) -> Self
//         where
//             F: FnOnce(ProjectionExpr<'a, DB>) -> ProjectionExpr<'a, DB> {
//         self.operand = f(self.operand);
//         self
//     }
// }

// pub enum ProjectionExpr<'a, DB: Database> {
//     Column(IdentifierMeta),
//     Function(Function<'a, DB>),
//     Subselect(Box<SelectStatement<'a, DB>>),
// }



// impl<'a, T, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Identifier<T> {
//     fn into_arg(self) -> Projection<'a, DB, X> {
//         Projection::new(ProjectionExpr::Column(self.into_identifier().meta), None)
//     }
// }

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
