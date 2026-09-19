use crate::dsl::assignment::ExtAssignment;
use crate::types::Sqlite;
use kova_core::dsl::{Assignment, Column, WithComponent};
use kova_core::types::IdentifierMeta;
use std::marker::PhantomData;

pub struct ExtColumn<T> {
    meta: IdentifierMeta,
    _marker: PhantomData<T>,
}

impl<T> ExtColumn<T> {
    #[doc(hidden)]
    pub const fn create<'a>(meta: IdentifierMeta) -> Column<'a, T, Sqlite> {
        Column::new(meta, Self::new(meta))
    }

    const fn new(meta: IdentifierMeta) -> ExtColumn<T> {
        Self { meta, _marker: PhantomData }
    }
}

impl<T> ExtColumn<T> {
    pub fn excluded<'a, X>(&self) -> Assignment<'a, Sqlite, X> {
        Assignment::from_ext(ExtAssignment::ExcludedColumn(self.meta))
    }
}
