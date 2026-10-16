use crate::dsl::{
    AstNode, FromItem, Identifier, IntoArg, IntoIdentifier,
    IntoOrdering, Operand, Ordering, SingleArgMarker, Static,
    TypedNode,
};
use crate::type_id;
use crate::types::{Database, IdentifierMeta};
use std::marker::PhantomData;

pub struct Table<T> {
    pub meta: IdentifierMeta,
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
    pub const fn new(source: &'static str, name: &'static str) -> Self {
        Self { meta: IdentifierMeta::new(source, name), _marker: PhantomData }
    }

    #[inline(always)]
    pub fn tag<Marker: 'static>(mut self) -> Self {
        self.meta = self.meta.with_tag(type_id::<Marker>());
        self
    }
}

impl<T> SingleArgMarker for Table<T> {}

impl<T> TypedNode for Table<T> {
    type Output = Static<T>;
}

impl<'a, DB: Database, T, X> AstNode<'a, DB, X> for Table<T> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, DB> {
        Operand::from_column(self.meta)
    }
}

impl<T> IntoIdentifier<T> for Table<T> {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.meta)
    }
}

impl<T> IntoArg<IdentifierMeta> for Table<T> {
    #[inline(always)]
    fn into_arg(self) -> IdentifierMeta {
        self.meta
    }
}

impl<'a, DB: Database, T, X> IntoOrdering<'a, DB, X> for Table<T> {
    fn into_ordering(self) -> Ordering<'a, DB, X> {
        Ordering::from_operand(Operand::from_column(self.meta))
    }
}

impl<'a, T, DB: Database, X> IntoArg<FromItem<'a, DB, X>> for Table<T> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, DB, X> {
        FromItem::from_table(self.meta.source, self.meta.tag)
    }
}
