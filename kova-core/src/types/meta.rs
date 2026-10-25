use crate::dsl::{Identifier, IntoIdentifier};
use std::any::TypeId;

// Outermost metadata: wraps `IdentifierMeta` with physical column attributes (`is_auto`).
#[derive(Debug, Copy, Clone)]
pub struct ColumnMeta {
    pub ident: IdentifierMeta,
    pub is_auto: bool,
}

impl ColumnMeta {
    #[inline(always)]
    pub const fn new(table: &'static str, name: &'static str, is_auto: bool) -> Self {
        Self { ident: IdentifierMeta::new(table, name), is_auto }
    }

    #[inline(always)]
    pub const fn set_tag(&mut self, tag: TypeId) {
        self.ident.set_tag(tag);
    }
}

// Middle metadata: wraps `TableMeta` with a column/identifier name.
#[derive(Debug, Copy, Clone)]
pub struct IdentifierMeta {
    pub table: TableMeta,
    pub name: &'static str,
}

impl IdentifierMeta {
    #[inline(always)]
    pub const fn new(table: &'static str, name: &'static str) -> Self {
        Self { table: TableMeta::new(table), name }
    }

    #[inline(always)]
    pub const fn set_tag(&mut self, tag: TypeId) {
        self.table.set_tag(tag);
    }
}

// Innermost metadata: represents a table/relation name and its optional alias tag.
#[derive(Debug, Copy, Clone)]
pub struct TableMeta {
    pub name: &'static str,
    pub tag: Option<TypeId>,
}

impl TableMeta {
    #[inline(always)]
    pub const fn new(name: &'static str) -> Self {
        Self { name, tag: None }
    }

    #[inline(always)]
    pub const fn set_tag(&mut self, tag: TypeId) {
        self.tag = Some(tag);
    }
}

pub trait SetTag {
    fn set_tag(&mut self, tag: TypeId);
}

impl SetTag for ColumnMeta {
    #[inline(always)]
    fn set_tag(&mut self, tag: TypeId) {
        Self::set_tag(self, tag);
    }
}

impl SetTag for IdentifierMeta {
    #[inline(always)]
    fn set_tag(&mut self, tag: TypeId) {
        Self::set_tag(self, tag);
    }
}

impl SetTag for TableMeta {
    #[inline(always)]
    fn set_tag(&mut self, tag: TypeId) {
        Self::set_tag(self, tag);
    }
}

impl<T> IntoIdentifier<T> for ColumnMeta {
    #[inline(always)]
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.ident)
    }
}

impl<T> IntoIdentifier<T> for IdentifierMeta {
    #[inline(always)]
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self)
    }
}
