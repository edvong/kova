use crate::dsl::function::Function;
use crate::dsl::statement::SelectStatement;
use crate::types::IdentifierMeta;
use crate::types::{Database, Encode};
use crate::{Error, Result};
use std::marker::PhantomData;

pub struct Operand<'a, DB: Database> {
    inner: OperandNode<'a, DB>,
}

#[derive(Copy, Clone)]
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div,
}

enum OperandNode<'a, DB: Database> {
    Value(DB::Value<'a>),
    Column(IdentifierMeta),
    Function(Function<'a, DB>),
    Subselect(Box<SelectStatement<'a, DB>>),
    Wildcard,
    Math(MathOperator, Box<Operand<'a, DB>>, Box<Operand<'a, DB>>),
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

    pub fn from_column(meta: IdentifierMeta) -> Self {
        Operand { inner: OperandNode::Column(meta) }
    }

    pub fn from_function(f: Function<'a, DB>) -> Self {
        Operand { inner: OperandNode::Function(f) }
    }

    pub fn from_select(s: SelectStatement<'a, DB>) -> Self {
        Operand { inner: OperandNode::Subselect(Box::new(s)) }
    }

    pub fn from_wildcard() -> Self {
        Operand { inner: OperandNode::Wildcard }
    }

    pub fn from_math(op: MathOperator, left: Self, right: Self) -> Self {
        Operand { inner: OperandNode::Math(op, Box::new(left), Box::new(right)) }
    }

    pub fn from_error(err: Error) -> Self {
        Operand { inner: OperandNode::Error(Box::new(err)) }
    }
}

pub struct JsonType;
pub struct JsonbType;

// Concrete type marker (handles any Rust type like primitives, Uuid, Url, etc.)
pub struct Static<T: ?Sized>(PhantomData<T>);

// Marker for dynamic/untyped stuff: SRF columns, JSON chains, subqueries
pub struct Dynamic;

// Helper to wrap Static<T> into Static<Vec<T>> (needed by ArrayAgg, etc.)
pub trait IntoArrayOutput {
    type Output;
}

impl<T> IntoArrayOutput for Static<T> {
    type Output = Static<Vec<T>>;
}

impl IntoArrayOutput for Dynamic {
    type Output = Dynamic;
}

pub trait TypedNode {
    type Output;
}

impl<T: TypedNode> TypedNode for Option<T> {
    type Output = T::Output;
}

// High-level: turns AST nodes into Operands directly (keeps raw literals out)
pub trait AstNode<'a, DB: Database, X>: TypedNode {
    fn into_ast_operand(self) -> Operand<'a, DB>;
}

// Marker: for internal AST wrappers only (Node<T>, Select, Any, All, etc.)
pub struct Close;

// Marker: blanket impl for arbitrary Rust values that implement Encode
pub struct Open;

// Low-level: converts to Operand while checking target type compatibility
#[diagnostic::on_unimplemented(
    message = "cannot use `{Self}` as an operand where `{Target}` is expected (under codec `{X}`)",
    label = "invalid or incompatible SQL operand",
    note = "ensure the operand type is compatible with `{Target}` and implements `Encode` under codec `{X}`."
)]
pub trait IntoOperand<'a, Target, DB: Database, K, X> {
    fn into_operand(self) -> Operand<'a, DB>;
}

impl<'a, T, Target, DB, X> IntoOperand<'a, Target, DB, Open, X> for T
    where
        DB: Encode<'a, T, X>,
        Static<T>: Compatible<Target, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_result(DB::encode(self))
    }
}

// ------------------------------------------
// Type compatibility matrix
// ------------------------------------------
#[diagnostic::on_unimplemented(
    message = "SQL type mismatch: `{Self}` is not compatible with `{Target}` (under codec `{X}`)",
    label = "incompatible SQL type",
    note = "if this is a custom cross-type comparison, implement `Compatible<{Target}, {X}> for {Self}`."
)]
pub trait Compatible<Target, X> {}

// 1. Static -> Static (Identity, References, Box, and Option Reference Penetration)
impl<T, X> Compatible<Static<T>, X> for Static<T> {}
impl<'a, T, X> Compatible<Static<T>, X> for Static<&'a T> {}
impl<'a, 'b, T, X> Compatible<Static<T>, X> for Static<&'b &'a T> {}
impl<T, X> Compatible<Static<T>, X> for Static<Box<T>> {}
impl<'a, T, X> Compatible<Static<T>, X> for Static<&'a Box<T>> {}

impl<T, X> Compatible<Static<Option<T>>, X> for Static<T> {}
impl<'a, T, X> Compatible<Static<Option<T>>, X> for Static<&'a T> {}
impl<'a, 'b, T, X> Compatible<Static<Option<T>>, X> for Static<&'b &'a T> {}
impl<T, X> Compatible<Static<Option<T>>, X> for Static<Box<T>> {}
impl<'a, T, X> Compatible<Static<Option<T>>, X> for Static<&'a Box<T>> {}
impl<'a, T, X> Compatible<Static<Option<T>>, X> for Static<Option<&'a T>> {}
impl<'a, 'b, T, X> Compatible<Static<Option<T>>, X> for Static<&'b Option<&'a T>> {}

// String & str / char compatibility
impl<'a, X> Compatible<Static<String>, X> for Static<&'a str> {}
impl<'a, 'b, X> Compatible<Static<String>, X> for Static<&'b &'a str> {}
impl<'a, X> Compatible<Static<Option<String>>, X> for Static<&'a str> {}
impl<'a, 'b, X> Compatible<Static<Option<String>>, X> for Static<&'b &'a str> {}
impl<'a, X> Compatible<Static<Option<String>>, X> for Static<Option<&'a str>> {}
impl<'a, 'b, X> Compatible<Static<Option<String>>, X> for Static<&'b Option<&'a str>> {}
impl<X> Compatible<Static<String>, X> for Static<char> {}
impl<'a, X> Compatible<Static<String>, X> for Static<&'a char> {}
impl<X> Compatible<Static<Option<String>>, X> for Static<char> {}
impl<'a, X> Compatible<Static<Option<String>>, X> for Static<&'a char> {}
impl<X> Compatible<Static<Option<String>>, X> for Static<Option<char>> {}

// Slice & Fixed-Array -> Vec<T> compatibility
impl<'a, T, X> Compatible<Static<Vec<T>>, X> for Static<&'a [T]> {}
impl<'a, 'b, T, X> Compatible<Static<Vec<T>>, X> for Static<&'b &'a [T]> {}
impl<'a, T, X> Compatible<Static<Option<Vec<T>>>, X> for Static<&'a [T]> {}
impl<'a, 'b, T, X> Compatible<Static<Option<Vec<T>>>, X> for Static<&'b &'a [T]> {}
impl<'a, T, X> Compatible<Static<Option<Vec<T>>>, X> for Static<Option<&'a [T]>> {}
impl<'a, 'b, T, X> Compatible<Static<Option<Vec<T>>>, X> for Static<&'b Option<&'a [T]>> {}

impl<T, const N: usize, X> Compatible<Static<Vec<T>>, X> for Static<[T; N]> {}
impl<'a, T, const N: usize, X> Compatible<Static<Vec<T>>, X> for Static<&'a [T; N]> {}
impl<T, const N: usize, X> Compatible<Static<Option<Vec<T>>>, X> for Static<[T; N]> {}
impl<'a, T, const N: usize, X> Compatible<Static<Option<Vec<T>>>, X> for Static<&'a [T; N]> {}
impl<T, const N: usize, X> Compatible<Static<Option<Vec<T>>>, X> for Static<Option<[T; N]>> {}
impl<'a, T, const N: usize, X> Compatible<Static<Option<Vec<T>>>, X>
    for Static<Option<&'a [T; N]>> {}

// JSON / JSONB compatibility
impl<'a, X> Compatible<Static<JsonbType>, X> for Static<&'a str> {}
impl<X> Compatible<Static<JsonbType>, X> for Static<String> {}
impl<'a, X> Compatible<Static<JsonbType>, X> for Static<&'a String> {}
impl<X> Compatible<Static<JsonbType>, X> for Static<i32> {}
impl<X> Compatible<Static<JsonbType>, X> for Static<bool> {}

impl<'a, X> Compatible<Static<JsonType>, X> for Static<&'a str> {}
impl<X> Compatible<Static<JsonType>, X> for Static<String> {}
impl<'a, X> Compatible<Static<JsonType>, X> for Static<&'a String> {}
impl<X> Compatible<Static<JsonType>, X> for Static<i32> {}
impl<X> Compatible<Static<JsonType>, X> for Static<bool> {}

macro_rules! impl_compatible_cast {
    ($from:ty =>$($to:ty),+) => {$(
        impl<X> Compatible<Static<$to>, X> for Static<$from> {}
        impl<'a, X> Compatible<Static<$to>, X> for Static<&'a$from> {}
        impl<X> Compatible<Static<Option<$to>>, X> for Static<$from> {}
        impl<'a, X> Compatible<Static<Option<$to>>, X> for Static<&'a$from> {}
        impl<X> Compatible<Static<Option<$to>>, X> for Static<Option<$from>> {}
        impl<'a, X> Compatible<Static<Option<$to>>, X> for Static<Option<&'a$from>> {}
        impl<'a, X> Compatible<Static<Option<$to>>, X> for Static<&'a Option<$from>> {}
    )+};
}

impl_compatible_cast!(i8  => i16, i32, i64, i128, isize, f32, f64);
impl_compatible_cast!(i16 => i32, i64, i128, isize, f32, f64);
impl_compatible_cast!(i32 => i64, i128, f64);
impl_compatible_cast!(i64 => i128);
impl_compatible_cast!(u8  => u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64);
impl_compatible_cast!(u16 => u32, u64, u128, usize, i32, i64, i128, f32, f64);
impl_compatible_cast!(u32 => u64, u128, usize, i64, i128, f64);
impl_compatible_cast!(u64 => u128);
impl_compatible_cast!(f32 => f64);

// 2. Static -> Dynamic
impl<T, X> Compatible<Dynamic, X> for Static<T> {}

// 3. Dynamic -> Static
impl<T, X> Compatible<Static<T>, X> for Dynamic {}

// 4. Dynamic -> Dynamic
impl<X> Compatible<Dynamic, X> for Dynamic {}
