use std::marker::PhantomData;

pub trait IntoIdentifier<T> {
    fn into_identifier(self) -> Identifier<T>;
}

impl<T> IntoIdentifier<T> for Identifier<T> {
    fn into_identifier(self) -> Identifier<T> {
        self
    }
}

pub struct Identifier<T> {
    pub meta: IdentifierMeta,
    _marker: PhantomData<T>,
}

#[derive(Copy, Clone)]
pub struct IdentifierMeta {
    pub source: &'static str,
    pub name: &'static str,
}

impl<T> Identifier<T> {
    pub const fn new(source: &'static str, name: &'static str) -> Self {
        Self { meta: IdentifierMeta { source, name }, _marker: PhantomData }
    }
}
