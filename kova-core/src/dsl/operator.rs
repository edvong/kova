use crate::dsl::component::Component;
use crate::dsl::expression::Expression;
use crate::types::{ColumnMeta, Database, Extension};

/// A unified operator node containing either a standard SQL operator
/// or a database-specific extension.
pub type Operator<'a, DB> = Component<
    StandardOperator<'a, DB>,
    <<DB as Database>::Extension<'a> as Extension<'a>>::Operator<'a>,
>;

pub enum StandardOperator<'a, DB: Database> {
    Eq(ColumnMeta, Expression<'a, DB>),
    Ne(ColumnMeta, Expression<'a, DB>),

    Gt(ColumnMeta, Expression<'a, DB>),
    Ge(ColumnMeta, Expression<'a, DB>),
    Lt(ColumnMeta, Expression<'a, DB>),
    Le(ColumnMeta, Expression<'a, DB>),

    IsNull(ColumnMeta),
    IsNotNull(ColumnMeta),

    Between(ColumnMeta, Expression<'a, DB>, Expression<'a, DB>),
    NotBetween(ColumnMeta, Expression<'a, DB>, Expression<'a, DB>),

    In(ColumnMeta, Vec<Expression<'a, DB>>),
    NotIn(ColumnMeta, Vec<Expression<'a, DB>>),

    Like(ColumnMeta, Expression<'a, DB>),
    NotLike(ColumnMeta, Expression<'a, DB>),
}
