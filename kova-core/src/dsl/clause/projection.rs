use crate::dsl::arg::{IntoArg, SingleArgMarker};
use crate::dsl::function::Function;
use crate::dsl::{Column, IntoArgs};
use crate::types::{ColumnMeta, Database};

pub struct ProjectionClause<'a, DB: Database> {
    items: Vec<Projection<'a, DB>>,
}

impl<'a, DB: Database> ProjectionClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub(crate) fn from(items: Vec<Projection<'a, DB>>) -> Self {
        Self { items }
    }
    pub fn set<I>(&mut self, args: I)
    where
        I: IntoArgs<Projection<'a, DB>>,
    {
        self.items.clear();
        self.extend(args);
    }

    pub fn extend<I>(&mut self, args: I)
    where
        I: IntoArgs<Projection<'a, DB>>,
    {
        self.items.reserve(args.size_hint());
        args.append_args(&mut self.items);
    }
}

pub struct Projection<'a, DB: Database> {
    pub(crate) expr: ProjectionExpr<'a, DB>,
    pub(crate) alias: Option<ColumnMeta>,
}

impl<'a, DB: Database> Projection<'a, DB> {
    pub fn new(expr: ProjectionExpr<'a, DB>, alias: Option<ColumnMeta>) -> Self {
        Self { expr, alias }
    }
}

pub enum ProjectionExpr<'a, DB: Database> {
    Column(ColumnMeta),
    Function(Function<'a, DB>),
}

impl<'a, DB: Database> SingleArgMarker for Projection<'a, DB> {}

impl<'a, T, DB: Database> IntoArg<Projection<'a, DB>> for Column<'a, T, DB> {
    fn into_arg(self) -> Projection<'a, DB> {
        Projection { expr: ProjectionExpr::Column(self.meta), alias: None }
    }
}

impl<'a, DB: Database> IntoArg<Projection<'a, DB>> for Function<'a, DB> {
    fn into_arg(self) -> Projection<'a, DB> {
        Projection { expr: ProjectionExpr::Function(self), alias: None }
    }
}
