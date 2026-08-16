use crate::dsl::expression::Expression;
use crate::render::Render;
use crate::types::{ColumnMeta, Database, Extension};

/// A unified operator node containing either a standard SQL operator
/// or a database-specific extension.
pub enum Operator<'a, DB: Database> {
    Standard(StandardOperator<'a, DB>),
    Extension(<DB::Extension<'a> as Extension<'a>>::Operator<'a>),
}

pub trait ExtensionOperator: Render {}

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


impl<'a, DB: Database> From<StandardOperator<'a, DB>> for Operator<'a, DB> {
    fn from(value: StandardOperator<'a, DB>) -> Self {
        Operator::Standard(value)
    }
}