use kova_core::types::ColumnMeta;
use std::marker::PhantomData;

pub struct Column<T> {
    meta: ColumnMeta,
    _marker: PhantomData<T>,
}

impl<T> Column<T> {
    pub const fn new(meta: ColumnMeta) -> Self {
        Self {
            meta,
            _marker: PhantomData,
        }
    }
}

impl<T> Column<T> {
    pub fn ext_operator(&self) {}
}
