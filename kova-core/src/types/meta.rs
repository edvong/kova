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

pub trait SyncMeta<T> {
    fn sync(&mut self, meta: T);
}

impl<T> SyncMeta<T> for () {
    fn sync(&mut self, _: T) { unreachable!() }
}
