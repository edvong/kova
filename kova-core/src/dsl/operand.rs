use crate::dsl::function::Function;
use crate::dsl::statement::SelectStatement;
use crate::types::IdentifierMeta;
use crate::types::{Database, Encode};
use crate::{Error, Result};

pub struct Operand<'a, DB: Database> {
    inner: OperandNode<'a, DB>,
}

enum OperandNode<'a, DB: Database> {
    Value(DB::Value<'a>),
    Column(IdentifierMeta),
    Function(Function<'a, DB>),
    Subselect(Box<SelectStatement<'a, DB>>),
    /// Captures conversion or building errors inline to avoid interrupting
    /// the fluent builder API chain, deferring error handling to execution time.
    Error(Box<Error>),
}

impl<'a, DB: Database> Operand<'a, DB> {
    pub fn from_result(r: Result<DB::Value<'a>>) -> Self {
        match r {
            Ok(v) => Self::from_value(v),
            Err(e) => Self::from_error(e),
        }
    }

    pub fn from_value(value: DB::Value<'a>) -> Self {
        Operand { inner: OperandNode::Value(value) }
    }

    pub fn from_identifier(meta: IdentifierMeta) -> Self {
        Operand { inner: OperandNode::Column(meta) }
    }

    pub fn from_function(f: Function<'a, DB>) -> Self {
        Operand { inner: OperandNode::Function(f) }
    }

    pub fn from_select(s: SelectStatement<'a, DB>) -> Self {
        Operand { inner: OperandNode::Subselect(Box::new(s)) }
    }

    pub fn from_error(err: Error) -> Self {
        Operand { inner: OperandNode::Error(Box::new(err)) }
    }
}

pub trait OpNode {
    type Input;
    type Output;
}

pub struct OpAst;
pub struct OpValue;

pub trait IntoOperand<'a, Target, DB: Database, K, X> {
    fn into_operand(self) -> Operand<'a, DB>;
}

impl<'a, T, DB, X> IntoOperand<'a, T, DB, OpValue, X> for T
    where
        DB: Database,
        T: Encode<'a, DB, X, T> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_result(self.encode())
    }
}

impl<'a, 'b, T, DB, X> IntoOperand<'a, T, DB, OpValue, X> for &'b T
    where
        DB: Database,
        &'b T: Encode<'a, DB, X, T> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_result(self.encode())
    }
}

// Allow String Column accept &str
impl<'a, 'b, DB: Database, X> IntoOperand<'a, String, DB, OpValue, X> for &'b str
    where
        &'b str: Encode<'a, DB, X, String> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_result(self.encode())
    }
}

// Allow Vec<u8> Column accept &[u8]
impl<'a, 'b, DB: Database, X> IntoOperand<'a, Vec<u8>, DB, OpValue, X> for &'b [u8]
    where
        &'b [u8]: Encode<'a, DB, X, Vec<u8>> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_result(self.encode())
    }
}
