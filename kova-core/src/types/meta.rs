use crate::dsl::{Identifier, IntoIdentifier};

pub trait IntoMeta {
    fn into_meta(self) -> Self;
}

pub trait SetTag {
    fn set_tag(&mut self, tag: usize);
}

#[derive(Debug, Copy, Clone)]
pub struct ColumnMeta {
    pub ident: IdentifierMeta,
    pub is_auto: bool,
}

impl ColumnMeta {
    pub const fn new(source: &'static str, name: &'static str, is_auto: bool) -> Self {
        Self { ident: IdentifierMeta::new(source, name), is_auto }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IdentifierMeta {
    pub source: &'static str,
    pub name: &'static str,
    pub tag: Option<usize>,
}

impl IdentifierMeta {
    pub const fn new(source: &'static str, name: &'static str) -> Self {
        Self { source, name, tag: None }
    }

    #[inline(always)]
    pub fn with_tag(mut self, tag: usize) -> Self {
        self.tag = Some(tag);
        self
    }
}

impl IntoMeta for ColumnMeta {
    fn into_meta(self) -> Self {
        self
    }
}

impl IntoMeta for IdentifierMeta {
    fn into_meta(self) -> Self {
        self
    }
}

impl SetTag for ColumnMeta {
    fn set_tag(&mut self, tag: usize) {
        self.ident.tag = Some(tag);
    }
}

impl SetTag for IdentifierMeta {
    fn set_tag(&mut self, tag: usize) {
        self.tag = Some(tag);
    }
}

impl<T> IntoIdentifier<T> for ColumnMeta {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self.ident)
    }
}

impl<T> IntoIdentifier<T> for IdentifierMeta {
    fn into_identifier(self) -> Identifier<T> {
        Identifier::from(self)
    }
}


// pub trait SyncMeta<T> {
//     fn sync(&mut self, meta: T);
// }
// 
// 
// 
// impl<T> SyncMeta<T> for () {
//     fn sync(&mut self, _: T) { unreachable!() }
// }
