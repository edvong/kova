use crate::dsl::arg::{IntoArg, IsSingleArg};
use crate::dsl::function::Function;
use crate::dsl::{Column, Component, IntoArgs};
use crate::types::{ColumnMeta, Database, Extension};

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
        I: IntoArgs<Projection<'a, DB>>
    {
        self.items.clear();
        self.extend(args);
    }

    pub fn extend<I>(&mut self, args: I)
    where
        I: IntoArgs<Projection<'a, DB>>
    {
        self.items.reserve(args.size_hint());
        args.append_args(&mut self.items);
    }
}


pub type Projection<'a, DB> = Component<
    StandardProjection<'a, DB>,
    <<DB as Database>::Extension<'a> as Extension<'a>>::Projection,
>;

pub enum StandardProjection<'a, DB: Database> {
    Column(ColumnMeta, Option<ColumnMeta>),
    Function(Function<'a, DB>, Option<ColumnMeta>),
}

impl<'a, DB: Database> IsSingleArg for Projection<'a, DB> {}

impl<'a, T, DB: Database> IntoArg<Projection<'a, DB>> for Column<'a, T, DB> {
    fn into_arg(self) -> Projection<'a, DB> {
        Projection::Standard(StandardProjection::Column(self.meta, None))
    }
}

impl<'a, DB: Database> IntoArg<Projection<'a, DB>> for Function<'a, DB> {
    fn into_arg(self) -> Projection<'a, DB> {
        Projection::Standard(StandardProjection::Function(self, None))
    }
}
