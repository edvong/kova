use crate::dsl::ExtAssignment;
use crate::types::Postgres;
use kova_core::dsl::{Assignment, VirtualColumn};
use kova_core::types::{ColumnMeta, SyncMeta};
use std::ops::Deref;

pub struct ExtColumn<T> {
    meta: ColumnMeta,
    inner: VirtualColumn<Postgres, T>
}

impl<T> Copy for ExtColumn<T> {}
impl<T> Clone for ExtColumn<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Deref for ExtColumn<T> {
    type Target = VirtualColumn<Postgres, T>;

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
    pub const fn new(meta: ColumnMeta, inner: VirtualColumn<Postgres, T>) -> ExtColumn<T> {
        Self { meta, inner }
    }

    pub fn excluded<'a, X>(self) -> Assignment<'a, Postgres, X> {
        Assignment::from(ExtAssignment::ExcludedColumn(self.meta.ident))
    }
}
