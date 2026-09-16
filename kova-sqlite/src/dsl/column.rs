use crate::dsl::assignment::Assignment;
use crate::types::Sqlite;
use kova_core::dsl::WithComponent;
use kova_core::types::ColumnMeta;
use std::marker::PhantomData;

pub struct Column<T> {
    meta: ColumnMeta,
    _marker: PhantomData<T>,
}

impl<T> Column<T> {
    #[doc(hidden)]
    pub const fn create<'a>(meta: ColumnMeta) -> kova_core::dsl::Column<'a, T, Sqlite> {
        kova_core::dsl::Column::new(meta, Self::new(meta))
    }

    const fn new(meta: ColumnMeta) -> Column<T> {
        Self { meta, _marker: PhantomData }
    }
}

impl<T> Column<T> {
    pub fn excluded<'a, X>(&self) -> kova_core::dsl::Assignment<'a, Sqlite, X> {
        kova_core::dsl::Assignment::from_ext(Assignment::ExcludedColumn(self.meta))
    }
}
