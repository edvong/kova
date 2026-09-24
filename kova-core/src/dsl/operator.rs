use crate::dsl::Operand;
use crate::dsl::component::{Component, WithComponent};
use crate::types::{Database, Extension};

pub type ExtOperator<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Operator<'a>;

/// A unified operator node containing either a standard SQL operator
/// or a database-specific extension.
pub struct Operator<'a, DB: Database> {
    inner: Component<StandardOperator<'a, DB>, ExtOperator<'a, DB>>,
}

impl<'a, DB: Database> WithComponent for Operator<'a, DB> {
    type Standard = StandardOperator<'a, DB>;
    type Extension = ExtOperator<'a, DB>;

    fn from_std(std: Self::Standard) -> Self {
        Self { inner: Component::standard(std) }
    }

    fn from_ext(ext: Self::Extension) -> Self {
        Self { inner: Component::extension(ext) }
    }
}

pub enum StandardOperator<'a, DB: Database> {
    Eq(Operand<'a, DB>, Operand<'a, DB>),
    Ne(Operand<'a, DB>, Operand<'a, DB>),

    Gt(Operand<'a, DB>, Operand<'a, DB>),
    Ge(Operand<'a, DB>, Operand<'a, DB>),
    Lt(Operand<'a, DB>, Operand<'a, DB>),
    Le(Operand<'a, DB>, Operand<'a, DB>),

    IsNull(Operand<'a, DB>),
    IsNotNull(Operand<'a, DB>),

    Between(Operand<'a, DB>, Operand<'a, DB>, Operand<'a, DB>),
    NotBetween(Operand<'a, DB>, Operand<'a, DB>, Operand<'a, DB>),

    InList(Operand<'a, DB>, Vec<Operand<'a, DB>>),
    NotInList(Operand<'a, DB>, Vec<Operand<'a, DB>>),

    InSelect(Operand<'a, DB>, Operand<'a, DB>),
    NotInSelect(Operand<'a, DB>, Operand<'a, DB>),

    Like(Operand<'a, DB>, Operand<'a, DB>),
    NotLike(Operand<'a, DB>, Operand<'a, DB>),
}
