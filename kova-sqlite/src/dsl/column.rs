use std::marker::PhantomData;
use kova_core::types::ColumnMeta;

pub struct Column<T> {
    meta: ColumnMeta,
    _marker: PhantomData<T>
}

impl<T> Column<T> {
    pub const fn new(meta: ColumnMeta) -> Self {
        Self { meta, _marker: PhantomData }
    }
}

impl<T> Column<T> {

    pub fn ext_operator(&self) {

    }

}