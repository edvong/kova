use crate::dsl::function::Function;
use crate::types::{ColumnMeta, Database, Encode};
use crate::{Error, Result};
use std::sync::Arc;
use crate::statement::SelectStatement;

/// Represents a generic operand within a SQL expression AST (Abstract Syntax Tree).
/// An operand can be a dynamically bound database value, a column reference,
/// or a deferred error captured during the query-building phase.
// #[derive(Debug, Clone)]
pub enum Operand<'a, DB: Database> {
    Value(DB::Value<'a>),
    Column(ColumnMeta),
    Function(Function<'a, DB>),
    Subselect(Box<SelectStatement<'a, DB>>),
    /// Captures conversion or building errors inline to avoid interrupting
    /// the fluent builder API chain, deferring error handling to execution time.
    Error(Arc<Error>),
}

impl<'a, DB: Database> From<ColumnMeta> for Operand<'a, DB> {
    fn from(value: ColumnMeta) -> Self {
        Operand::Column(value)
    }
}

impl<'a, DB: Database> From<Result<DB::Value<'a>>> for Operand<'a, DB> {
    fn from(value: Result<DB::Value<'a>>) -> Self {
        match value {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}

// pub trait BuildOperand<'a, DB: Database> {
//     fn build_operand(self) -> Operand<'a, DB>;
// }


pub struct IsAst;
pub struct IsValue;

pub trait IntoOperand<'a, Target, DB: Database, K, X> {
    fn into_operand(self) -> Operand<'a, DB>;
}

impl<'a, T, DB, X> IntoOperand<'a, T, DB, IsValue, X> for T
where
    DB: Database,
    T: Encode<'a, DB, X, T>,
{
    fn into_operand(self) -> Operand<'a, DB> {
        match self.encode() {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}

impl<'a, 'b, T, DB, X> IntoOperand<'a, T, DB, IsValue, X> for &'b T
where
    DB: Database,
    &'b T: Encode<'a, DB, X, T>,
{
    fn into_operand(self) -> Operand<'a, DB> {
        match self.encode() {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}


// Allow String Column accept &str
impl<'a, 'b, DB: Database, X> IntoOperand<'a, String, DB, IsValue, X> for &'b str
where
    &'b str: Encode<'a, DB, X, String>
{
    fn into_operand(self) -> Operand<'a, DB> {
        match self.encode() {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}

// Allow Vec<u8> Column accept &[u8]
impl<'a, 'b, DB: Database, X> IntoOperand<'a, Vec<u8>, DB, IsValue, X> for &'b [u8]
where
    &'b [u8]: Encode<'a, DB, X, Vec<u8>>,
{
    fn into_operand(self) -> Operand<'a, DB> {
        match self.encode() {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}
