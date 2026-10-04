use crate::dsl::EraseX;
use crate::dsl::arg::SingleArgMarker;
use crate::dsl::component::{Component, Ext, Std};
use crate::dsl::operand::Operand;
use crate::types::{Database, Extension, IdentifierMeta};
use std::marker::PhantomData;

pub type ExtAssignment<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Assignment<'a>;

pub struct Assignment<'a, DB: Database, X> {
    inner: Component<StandardAssignment<'a, DB>, ExtAssignment<'a, DB>>,
    _marker: PhantomData<X>,
}

pub enum StandardAssignment<'a, DB: Database> {
    Value(IdentifierMeta, Operand<'a, DB>), // `col = ?` or `col = other_col`
    Increment(IdentifierMeta, Operand<'a, DB>), // `col = col + val`
    Decrement(IdentifierMeta, Operand<'a, DB>), // `col = col - val`
}

impl<'a, DB: Database, X> Assignment<'a, DB, X> {
    pub fn from<T, M>(a: T) -> Self 
        where 
            T: IntoAssignment<'a, DB, M, X> {
        a.into_assignment()
    }
}

impl<'a, DB: Database, X> SingleArgMarker for Assignment<'a, DB, X> {}

impl<'a, DB: Database, X> EraseX for Assignment<'a, DB, X> {
    type Erased = Assignment<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Assignment { inner: self.inner, _marker: PhantomData }
    }
}

pub trait IntoAssignment<'a, DB: Database, Marker, X> {
    fn into_assignment(self) -> Assignment<'a, DB, X>;
}

impl<'a, DB: Database, X> IntoAssignment<'a, DB, Std, X> for StandardAssignment<'a, DB> {
    fn into_assignment(self) -> Assignment<'a, DB, X> {
        Assignment { inner: Component::standard(self), _marker: PhantomData }
    }
}

impl<'a, DB: Database, X> IntoAssignment<'a, DB, Ext, X> for ExtAssignment<'a, DB> {
    fn into_assignment(self) -> Assignment<'a, DB, X> {
        Assignment { inner: Component::extension(self), _marker: PhantomData }
    }
}
