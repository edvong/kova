use std::marker::PhantomData;

pub trait IntoAlias<T> {
    fn into_identifier(self) -> Identifier;
}

pub struct Identifier {
    pub identifier: &'static str,
}

pub struct Alias<T> {
    pub column_name: &'static str,
    _marker: PhantomData<T>,
}

impl<T> Alias<T> {
    pub const fn new(column_name: &'static str) -> Self {
        Self { column_name, _marker: PhantomData }
    }
}

impl<T> IntoAlias<T> for Alias<T> {
    fn into_identifier(self) -> Identifier {
        Identifier { identifier: self.column_name }
    }
}
