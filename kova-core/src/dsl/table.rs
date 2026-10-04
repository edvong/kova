use crate::dsl::{
    FromItem, Identifier, IntoArg, IntoIdentifier, IntoOperand, IntoOrdering, IntoProjection,
    OpAst, OpNode, Operand, Ordering, Projection, SingleArgMarker,
};
use crate::type_id;
use crate::types::{Database, IdentifierMeta};
use std::marker::PhantomData;

pub struct Table<T> {
    pub meta: IdentifierMeta,
    _marker: PhantomData<T>,
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

impl<T> OpNode for Table<T> {
    type Input = ();
    type Output = T;
}

impl<'a, T, DB: Database, X> IntoOperand<'a, T, DB, OpAst, X> for Table<T> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, DB> {
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

impl<'a, DB: Database, T, X> IntoProjection<'a, DB, X> for Table<T> {
    fn into_projection(self) -> Projection<'a, DB, X> {
        Projection::new(IntoOperand::<T, DB, OpAst, X>::into_operand(self))
    }
}

impl<'a, T, DB: Database, X> IntoArg<FromItem<'a, DB, X>> for Table<T> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, DB, X> {
        FromItem::from_table(self.meta.source, self.meta.tag)
    }
}

impl<'a, T, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Table<T> {
    fn into_arg(self) -> Projection<'a, DB, X> {
        self.into_projection()
    }
}
