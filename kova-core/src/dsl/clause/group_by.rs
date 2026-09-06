use std::marker::PhantomData;
use crate::dsl::arg::{IntoArg, SingleArgMarker};
use crate::dsl::function::Function;
use crate::dsl::{Column, Component, EraseX};
use crate::types::{ColumnMeta, Database, Extension};

pub struct GroupByClause<'a, DB: Database> {
    items: Vec<GroupBy<'a, DB, ()>>,
}

impl<'a, DB: Database> GroupByClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn from(items: Vec<GroupBy<'a, DB, ()>>) -> Self {
        Self { items }
    }

    pub fn set(&mut self, args: Vec<GroupBy<'a, DB, ()>>) {
        self.items = args;
    }
}


pub struct GroupBy<'a, DB: Database, X> {
    pub(crate) inner: Component<
        StandardGroupBy<'a, DB>,
        <<DB as Database>::Extension<'a> as Extension<'a>>::GroupBy,
    >,
    _marker: PhantomData<X>
}

impl<'a, DB: Database, X> EraseX for GroupBy<'a, DB, X> {
    type Erased = GroupBy<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        GroupBy {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}


impl<'a, DB: Database, X> GroupBy<'a, DB, X> {
    pub fn standard(std: StandardGroupBy<'a, DB>) -> Self {
        Self { inner: Component::standard(std), _marker: PhantomData }
    }

    pub fn extension(ext: <<DB as Database>::Extension<'a> as Extension<'a>>::GroupBy) -> Self {
        Self { inner: Component::extension(ext), _marker: PhantomData }
    }
}


// Core difference from Projection: only columns and functions, no Option<ColumnMeta> alias
pub enum StandardGroupBy<'a, DB: Database> {
    Column(ColumnMeta),
    Function(Function<'a, DB>),
}


impl<'a, DB: Database, X> SingleArgMarker for GroupBy<'a, DB, X> {}

impl<'a, T, DB: Database, X> IntoArg<GroupBy<'a, DB, X>> for Column<'a, T, DB> {
    fn into_arg(self) -> GroupBy<'a, DB, X> {
        GroupBy::standard(StandardGroupBy::Column(self.meta))
    }
}

impl<'a, DB: Database, X> IntoArg<GroupBy<'a, DB, X>> for Function<'a, DB> {
    fn into_arg(self) -> GroupBy<'a, DB, X> {
        GroupBy::standard(StandardGroupBy::Function(self))
    }
}
