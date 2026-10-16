use crate::dsl::can::CanGroup;
use crate::dsl::identifier::{Identifier, IntoIdentifier};
use crate::dsl::operand::{Dynamic, Static};
use crate::dsl::{
    AstNode, CanAlias, CanAssignment, CanCompare, CanMath, CanOrder, CanString, Operand, TypedNode,
};
use crate::types::{ColumnMeta, Database, IdentifierMeta, SetTag};
use std::marker::PhantomData;

pub struct Column<T, M = ColumnMeta, Mode = Static<T>> {
    meta: M,
    _marker: PhantomData<(T, Mode)>,
}

impl<T, M: Copy, Mode> Copy for Column<T, M, Mode> {}
impl<T, M: Copy, Mode> Clone for Column<T, M, Mode> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Column<T, ColumnMeta, Static<T>> {
    pub const fn new(source: &'static str, name: &'static str, is_auto: bool) -> Self {
        Column { meta: ColumnMeta::new(source, name, is_auto), _marker: PhantomData }
    }
}

impl<T> Column<T, IdentifierMeta, Static<T>> {
    pub const fn identifier(source: &'static str, name: &'static str) -> Self {
        Self { meta: IdentifierMeta::new(source, name), _marker: PhantomData }
    }
}

impl Column<Dynamic, IdentifierMeta, Dynamic> {
    pub const fn dynamic(source: &'static str, name: &'static str) -> Self {
        Self { meta: IdentifierMeta::new(source, name), _marker: PhantomData }
    }
}

impl<T, M, Mode> SetTag for Column<T, M, Mode>
    where
        M: SetTag {
    fn set_tag(&mut self, tag: usize) {
        self.meta.set_tag(tag);
    }
}

impl<T, M, Mode> Column<T, M, Mode>
    where
        M: IntoIdentifier<T> + Copy {
    #[inline(always)]
    pub fn source(&self) -> &'static str {
        self.meta.into_identifier().meta.source
    }
}

impl<T, M, Mode> TypedNode for Column<T, M, Mode> {
    type Output = Mode;
}

impl<'a, T, M, Mode> IntoIdentifier<T> for Column<T, M, Mode>
    where
        M: IntoIdentifier<T> {
    fn into_identifier(self) -> Identifier<T> {
        self.meta.into_identifier()
    }
}

impl<'a, DB: Database, T, M, Mode, X> AstNode<'a, DB, X> for Column<T, M, Mode>
    where
        M: IntoIdentifier<T> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, DB> {
        Operand::from_column(self.meta.into_identifier().meta)
    }
}

impl<T, M, Mode> CanCompare for Column<T, M, Mode> {}
impl<T, M, Mode> CanOrder for Column<T, M, Mode> {}
impl<T, M, Mode> CanGroup for Column<T, M, Mode> {}
impl<T, M, Mode> CanAlias for Column<T, M, Mode> {}
impl<T, M, Mode> CanMath for Column<T, M, Mode> {}
impl<T, M, Mode> CanString for Column<T, M, Mode> {}
// Only the physical column (ColumnMeta) has the ability to be assigned values.
impl<T> CanAssignment for Column<T, ColumnMeta, Static<T>> {}
