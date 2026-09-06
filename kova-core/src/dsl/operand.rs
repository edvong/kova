use crate::dsl::column::Column;
use crate::dsl::function::Function;
use crate::types::{ColumnMeta, Database, IntoValue};
use crate::{Error, Result};
use std::sync::Arc;

/// Represents a generic operand within a SQL expression AST (Abstract Syntax Tree).
/// An operand can be a dynamically bound database value, a column reference,
/// or a deferred error captured during the query-building phase.
// #[derive(Debug, Clone)]
pub enum Operand<'a, DB: Database> {
    Value(DB::Value<'a>),
    Column(ColumnMeta),
    Function(Function<'a, DB>),
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

/// A conversion trait used to normalize various types into a unified `Operand`.
/// This trait enables the SQL DSL to seamlessly accept both explicit database columns
/// and native Rust values in its expression methods.
pub trait IntoOperand<'a, Target, DB: Database> {
    fn into_operand(self) -> Operand<'a, DB>;
}

impl<'a, T, DB: Database> IntoOperand<'a, T, DB> for Column<'a, T, DB> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::Column(self.meta)
    }
}

/// Blanket implementation for any type `T` that implements `IntoValue`.
///
/// **Note on Lifetimes and References:**
/// In Rust, a reference (e.g., `&'a str` or `&'a T`) is considered a distinct type `T`.
/// Because `IntoValue::into_value(self)` takes ownership of `self`, this single generic
/// implementation inherently covers both owned types and borrowed reference types.
/// Therefore, redundant implementations specifically targeting `&'a T`, `&'a str`,
/// or `&'a [u8]` are completely unnecessary and intentionally omitted.
impl<'a, T, DB> IntoOperand<'a, T, DB> for T
where
    DB: Database,
    T: IntoValue<'a, DB>,
{
    fn into_operand(self) -> Operand<'a, DB> {
        match self.into_value() {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}

impl<'a, 'b, T, DB> IntoOperand<'a, T, DB> for &'b T
where
    DB: Database,
    &'b T: IntoValue<'a, DB>,
{
    fn into_operand(self) -> Operand<'a, DB> {
        match self.into_value() {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}


// Allow String Column accept &str
impl<'a, 'b, DB: Database> IntoOperand<'a, String, DB> for &'b str
where
    &'b str: IntoValue<'a, DB>,
{
    fn into_operand(self) -> Operand<'a, DB> {
        match self.into_value() {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}

// Allow Vec<u8> Column accept &[u8]
impl<'a, 'b, DB: Database> IntoOperand<'a, Vec<u8>, DB> for &'b [u8]
where
    &'b [u8]: IntoValue<'a, DB>,
{
    fn into_operand(self) -> Operand<'a, DB> {
        match self.into_value() {
            Ok(v) => Operand::Value(v),
            Err(e) => Operand::Error(Arc::new(e)),
        }
    }
}
