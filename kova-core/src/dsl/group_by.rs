use crate::dsl::arg::{IntoArg, IsSingleArg};
use crate::dsl::function::Function;
use crate::dsl::{Column, Component, IntoArgs};
use crate::types::{ColumnMeta, Database, Extension};

pub struct GroupByClause<'a, DB: Database> {
    items: Vec<GroupBy<'a, DB>>,
}

impl<'a, DB: Database> GroupByClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn from(items: Vec<GroupBy<'a, DB>>) -> Self {
        Self { items }
    }

    pub fn set<I>(&mut self, args: I)
    where
        I: IntoArgs<GroupBy<'a, DB>>
    {
        self.items.clear();
        self.extend(args);
    }

    pub fn extend<I>(&mut self, args: I)
    where
        I: IntoArgs<GroupBy<'a, DB>>
    {
        self.items.reserve(args.size_hint());
        args.append_args(&mut self.items);
    }
}

// Specific extensions to GroupBy are reserved for each database.
pub type GroupBy<'a, DB> = Component<
    StandardGroupBy<'a, DB>,
    <<DB as Database>::Extension<'a> as Extension<'a>>::GroupBy,
>;

// Core difference from Projection: only columns and functions, no Option<ColumnMeta> alias
pub enum StandardGroupBy<'a, DB: Database> {
    Column(ColumnMeta),
    Function(Function<'a, DB>),
}


impl<'a, DB: Database> IsSingleArg for GroupBy<'a, DB> {}

impl<'a, T, DB: Database> IntoArg<GroupBy<'a, DB>> for Column<'a, T, DB> {
    fn into_arg(self) -> GroupBy<'a, DB> {
        GroupBy::Standard(StandardGroupBy::Column(self.meta))
    }
}

impl<'a, DB: Database> IntoArg<GroupBy<'a, DB>> for Function<'a, DB> {
    fn into_arg(self) -> GroupBy<'a, DB> {
        GroupBy::Standard(StandardGroupBy::Function(self))
    }
}
