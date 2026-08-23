mod operand;

use crate::dsl::component::Component;
use crate::dsl::operator::operand::Operand;
use crate::types::{ColumnMeta, Database, Extension};

pub use operand::IntoOperand;

/// A unified operator node containing either a standard SQL operator
/// or a database-specific extension.
pub type Operator<'a, DB> = Component<
    StandardOperator<'a, DB>,
    <<DB as Database>::Extension<'a> as Extension<'a>>::Operator<'a>,
>;

pub enum StandardOperator<'a, DB: Database> {
    Eq(ColumnMeta, Operand<'a, DB>),
    Ne(ColumnMeta, Operand<'a, DB>),

    Gt(ColumnMeta, Operand<'a, DB>),
    Ge(ColumnMeta, Operand<'a, DB>),
    Lt(ColumnMeta, Operand<'a, DB>),
    Le(ColumnMeta, Operand<'a, DB>),

    IsNull(ColumnMeta),
    IsNotNull(ColumnMeta),

    Between(ColumnMeta, Operand<'a, DB>, Operand<'a, DB>),
    NotBetween(ColumnMeta, Operand<'a, DB>, Operand<'a, DB>),

    In(ColumnMeta, Vec<Operand<'a, DB>>),
    NotIn(ColumnMeta, Vec<Operand<'a, DB>>),

    Like(ColumnMeta, Operand<'a, DB>),
    NotLike(ColumnMeta, Operand<'a, DB>),
}
