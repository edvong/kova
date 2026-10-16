#![allow(clippy::wrong_self_convention)]

use crate::types::IdentifierMeta;
use std::marker::PhantomData;

pub trait IntoIdentifier<T> {
    fn into_identifier(self) -> Identifier<T>;
}

impl<T> IntoIdentifier<T> for Identifier<T> {
    fn into_identifier(self) -> Identifier<T> {
        self
    }
}

impl<T> IntoIdentifier<T> for &Identifier<T> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier { meta: self.meta, _marker: PhantomData }
    }
}

pub struct Identifier<T> {
    pub meta: IdentifierMeta,
    _marker: PhantomData<T>,
}

impl<T> Copy for Identifier<T> {}
impl<T> Clone for Identifier<T> { fn clone(&self) -> Self { *self } }


// impl<T> SingleArgMarker for Identifier<T> {}

impl<T> Identifier<T> {
    pub const fn from(meta: IdentifierMeta) -> Self {
        Self { meta, _marker: PhantomData }
    }

    pub const fn new(source: &'static str, name: &'static str) -> Self {
        Self { meta: IdentifierMeta::new(source, name), _marker: PhantomData }
    }

    // For CTE and View
    // #[inline(always)]
    // pub fn tag<Marker: 'static>(mut self) -> Self {
    //     self.meta = self.meta.with_tag(type_id::<Marker>());
    //     self
    // }
}
