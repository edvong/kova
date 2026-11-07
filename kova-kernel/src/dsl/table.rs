use crate::dsl::{AstNode, Operand, Static, TypedNode};
use crate::types::{Database, TableMeta};
use std::any::TypeId;
use std::marker::PhantomData;

pub struct Table<T> {
    pub meta: TableMeta,
    _marker: PhantomData<T>,
}

impl<T> Copy for Table<T> {}
impl<T> Clone for Table<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Table<T> {
    #[inline(always)]
    pub const fn new(name: &'static str) -> Self {
        Self { meta: TableMeta::new(name), _marker: PhantomData }
    }

    #[inline(always)]
    pub const fn tag<Marker: 'static>(mut self) -> Self {
        self.meta.set_tag(TypeId::of::<Marker>());
        self
    }
}

impl<T> TypedNode for Table<T> {
    type Output = Static<T>;
}

impl<'a, DB: Database, T, X> AstNode<'a, DB, X> for Table<T> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, DB> {
        Operand::from_table(self.meta)
    }
}