use crate::dsl::operand::Operand;
use crate::dsl::component::{Component, WithComponent};
use crate::types::{ColumnMeta, Database, Extension};

pub type ExtAssignment<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Assignment<'a>;

pub struct Assignment<'a, DB: Database> {
    pub(crate) inner: Component<StandardAssignment<'a, DB>, ExtAssignment<'a, DB>>,
}

impl<'a, DB: Database> WithComponent for Assignment<'a, DB> {
    type Standard = StandardAssignment<'a, DB>;
    type Extension = ExtAssignment<'a, DB>;

    fn from_std(std: Self::Standard) -> Self {
        Self { inner: Component::standard(std) }
    }

    fn from_ext(ext: Self::Extension) -> Self {
        Self { inner: Component::extension(ext) }
    }
}

pub enum StandardAssignment<'a, DB: Database> {
    // `col = ?` or `col = other_col`
    Value(ColumnMeta, Operand<'a, DB>),
}