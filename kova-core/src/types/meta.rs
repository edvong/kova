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
}

impl IdentifierMeta {
    pub const fn new(source: &'static str, name: &'static str) -> Self {
        Self { source, name }
    }
}
