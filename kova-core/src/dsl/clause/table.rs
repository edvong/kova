use crate::types::{Database, Extension};

type TableFunction<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::TableFunction<'a>;

pub struct TableClause<'a, DB: Database> {
    items: Vec<TableItem<'a, DB>>,
}

// pub trait IntoTableItem<'a, DB: Database, X> {
//     fn into_table_item(self) -> TableItem<'a, DB>;
// }
//
// impl<'a, DB: Database, X> IntoTableItem<'a, DB, X> for &'static str {
//     #[inline(always)]
//     fn into_table_item(self) -> TableItem<'a, DB> {
//         TableItem::Table(self)
//     }
// }

pub enum TableItem<'a, DB: Database> {
    Table(&'static str),
    Function(TableFunction<'a, DB>),
}

impl<'a, DB: Database> TableClause<'a, DB> {
    pub(crate) fn new_with(initial: &'static str) -> Self {
        Self { items: vec![TableItem::Table(initial)] }
    }

    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: TableItem<'a, DB>) {
        self.items.push(item);
    }

    pub fn add_table(&mut self, table: &'static str) {
        self.items.push(TableItem::Table(table));
    }

    pub fn extend<I: IntoIterator<Item = TableItem<'a, DB>>>(&mut self, iter: I) {
        self.items.extend(iter)
    }

    // pub fn add_function(&mut self, func: TableFunction<'a, DB>) {
    //     self.items.push(TableItem::Function(func));
    // }
}
