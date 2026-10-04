use crate::dsl::assignment::ExtAssignment;
use crate::types::Sqlite;
use kova_core::dsl::{Assignment, VirtualColumn};
use kova_core::types::{ColumnMeta, SyncMeta};
use std::ops::Deref;

pub struct ExtColumn<T> {
    meta: ColumnMeta,
    inner: VirtualColumn<Sqlite, T>,
}

impl<T> Copy for ExtColumn<T> {}
impl<T> Clone for ExtColumn<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Deref for ExtColumn<T> {
    type Target = VirtualColumn<Sqlite, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> SyncMeta<ColumnMeta> for ExtColumn<T> {
    fn sync(&mut self, meta: ColumnMeta) {
        self.meta = meta;
        self.inner.sync(meta.ident);
    }
}

impl<T> ExtColumn<T> {
    #[doc(hidden)]
    pub const fn new(meta: ColumnMeta, inner: VirtualColumn<Sqlite, T>) -> ExtColumn<T> {
        Self { meta, inner }
    }

    pub fn excluded<'a, X>(&self) -> Assignment<'a, Sqlite, X> {
        Assignment::from(ExtAssignment::ExcludedColumn(self.meta.ident))
    }
}
