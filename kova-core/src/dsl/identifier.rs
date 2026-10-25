use crate::types::IdentifierMeta;
use std::marker::PhantomData;

pub trait IntoIdentifier<T> {
    fn into_identifier(self) -> Identifier<T>;
}

impl<T> IntoIdentifier<T> for Identifier<T> {
    #[inline(always)]
    fn into_identifier(self) -> Identifier<T> {
        self
    }
}

impl<T> IntoIdentifier<T> for &Identifier<T> {
    #[inline(always)]
    fn into_identifier(self) -> Identifier<T> {
        Identifier { meta: self.meta, _marker: PhantomData }
    }
}

pub struct Identifier<T> {
    pub meta: IdentifierMeta,
    _marker: PhantomData<T>,
}

impl<T> Copy for Identifier<T> {}
impl<T> Clone for Identifier<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Identifier<T> {
    #[inline(always)]
    pub const fn from(meta: IdentifierMeta) -> Self {
        Self { meta, _marker: PhantomData }
    }

    #[inline(always)]
    pub const fn new(name: &'static str) -> Self {
        Self { meta: IdentifierMeta::new("", name), _marker: PhantomData }
    }
}