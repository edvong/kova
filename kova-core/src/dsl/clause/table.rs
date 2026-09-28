use crate::dsl::{EraseX, SingleArgMarker};
use crate::types::{Database, Extension};
use std::marker::PhantomData;

type TableFunction<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::TableFunction<'a>;

pub struct TableClause<'a, DB: Database> {
    items: Vec<Table<'a, DB, ()>>,
}

pub struct Table<'a, DB: Database, X> {
    inner: TableInner<'a, DB>,
    _marker: PhantomData<X>,
}

enum TableInner<'a, DB: Database> {
    Table(&'static str, Option<usize>),
    Function(TableFunction<'a, DB>, Option<usize>),
}

impl<'a, DB: Database, X> Table<'a, DB, X> {
    pub fn from_table(name: &'static str, tag: Option<usize>) -> Self {
        Self { inner: TableInner::Table(name, tag), _marker: PhantomData }
    }

    pub fn from_function(func: TableFunction<'a, DB>, tag: Option<usize>) -> Self {
        Self { inner: TableInner::Function(func, tag), _marker: PhantomData }
    }

    fn tag(&mut self, tag: usize) {
        match &mut self.inner {
            TableInner::Table(_, t) => *t = Some(tag),
            TableInner::Function(_, t) => *t = Some(tag),
        }
    }
}

impl<'a, DB: Database, X> EraseX for Table<'a, DB, X> {
    type Erased = Table<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Table { inner: self.inner, _marker: PhantomData }
    }
}

impl<'a, DB: Database, X> SingleArgMarker for Table<'a, DB, X> {}


impl<'a, DB: Database> TableClause<'a, DB> {
    pub(crate) fn new_with(table_name: &'static str) -> Self {
        Self { items: vec![Table::from_table(table_name, None)] }
    }

    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, table: Table<'a, DB, ()>) {
        self.items.push(table);
    }

    pub fn add_table(&mut self, table: &'static str) {
        self.items.push(Table::from_table(table, None));
    }

    pub fn extend<I: IntoIterator<Item = Table<'a, DB, ()>>>(&mut self, iter: I) {
        self.items.extend(iter)
    }

    pub fn tag_latest(&mut self, tag: usize) {
        if let Some(item) = self.items.last_mut() {
            item.tag(tag);
        }
    }

    // pub fn add_function(&mut self, func: TableFunction<'a, DB>) {
    //     self.items.push(TableItem::Function(func));
    // }
}
