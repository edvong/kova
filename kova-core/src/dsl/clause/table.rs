use crate::dsl::EraseX;
use crate::types::{Database, Extension};
use std::any::TypeId;
use std::marker::PhantomData;

pub type TableFunction<'a, DB> = <<DB as Database>::Extension as Extension>::TableFunction<'a>;

pub struct TableClause<'a, DB: Database> {
    pub(crate) items: Vec<FromItem<'a, DB, ()>>,
}

pub struct FromItem<'a, DB: Database, X> {
    pub(crate) inner: TableInner<'a, DB>,
    _marker: PhantomData<X>,
}

pub(crate) enum TableInner<'a, DB: Database> {
    Table(&'static str, Option<TypeId>),
    Function(TableFunction<'a, DB>, Option<TypeId>),
}

impl<'a, DB: Database, X> FromItem<'a, DB, X> {
    pub fn from_table(name: &'static str, tag: Option<TypeId>) -> Self {
        Self { inner: TableInner::Table(name, tag), _marker: PhantomData }
    }

    pub fn from_function(func: TableFunction<'a, DB>, tag: Option<TypeId>) -> Self {
        Self { inner: TableInner::Function(func, tag), _marker: PhantomData }
    }

    fn tag(&mut self, tag: TypeId) {
        match &mut self.inner {
            TableInner::Table(_, t) => *t = Some(tag),
            TableInner::Function(_, t) => *t = Some(tag),
        }
    }

    #[inline(always)]
    pub(crate) fn get_tag(&self) -> Option<TypeId> {
        match &self.inner {
            TableInner::Table(_, t) | TableInner::Function(_, t) => *t,
        }
    }
}

impl<'a, DB: Database, X> EraseX for FromItem<'a, DB, X> {
    type Erased = FromItem<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        FromItem { inner: self.inner, _marker: PhantomData }
    }
}

impl<'a, DB: Database> TableClause<'a, DB> {
    pub(crate) fn new_with(table_name: &'static str) -> Self {
        Self { items: vec![FromItem::from_table(table_name, None)] }
    }

    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn set_tables(&mut self, tables: Vec<&'static str>) {
        self.items.clear();
        self.items.reserve(tables.len());
        for table in tables {
            self.items.push(FromItem::from_table(table, None));
        }
    }

    pub fn extend(&mut self, items: Vec<FromItem<'a, DB, ()>>) {
        self.items.extend(items)
    }

    pub fn tag_latest(&mut self, tag: TypeId) {
        if let Some(item) = self.items.last_mut() {
            item.tag(tag);
        }
    }
}
