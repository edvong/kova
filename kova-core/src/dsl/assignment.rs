use crate::dsl::EraseX;
use crate::dsl::arg::SingleArgMarker;
use crate::dsl::component::{Component, WithComponent};
use crate::dsl::operand::Operand;
use crate::types::{Database, Extension, IdentifierMeta};
use std::marker::PhantomData;

pub type ExtAssignment<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Assignment<'a>;

pub struct Assignment<'a, DB: Database, X> {
    inner: Component<StandardAssignment<'a, DB>, ExtAssignment<'a, DB>>,
    _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> EraseX for Assignment<'a, DB, X> {
    type Erased = Assignment<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Assignment { inner: self.inner, _marker: PhantomData }
    }
}

impl<'a, DB: Database, X> WithComponent for Assignment<'a, DB, X> {
    type Standard = StandardAssignment<'a, DB>;
    type Extension = ExtAssignment<'a, DB>;

    fn from_std(std: Self::Standard) -> Self {
        Self { inner: Component::standard(std), _marker: PhantomData }
    }

    fn from_ext(ext: Self::Extension) -> Self {
        Self { inner: Component::extension(ext), _marker: PhantomData }
    }
}

pub enum StandardAssignment<'a, DB: Database> {
    Value(IdentifierMeta, Operand<'a, DB>), // `col = ?` or `col = other_col`
    Increment(IdentifierMeta, Operand<'a, DB>), // `col = col + val`
    Decrement(IdentifierMeta, Operand<'a, DB>), // `col = col - val`
}

impl<'a, DB: Database, X> SingleArgMarker for Assignment<'a, DB, X> {}
