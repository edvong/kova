use crate::types::{Sqlite, Value};
use kova_core::dsl::{Column, IntoArg, Operand};
use std::borrow::Cow;

mod json_each;
#[rustfmt::skip]
mod json_tree;

pub use json_each::JsonEach;
pub use json_tree::JsonTree;

pub enum ExtTableFunction<'a> {
    JsonEach(Operand<'a, Sqlite>, Option<Operand<'a, Sqlite>>),
    JsonTree(Operand<'a, Sqlite>, Option<Operand<'a, Sqlite>>),
}

pub struct JsonTableArgs<'a> {
    pub base: Operand<'a, Sqlite>,
    pub path: Option<Operand<'a, Sqlite>>,
}

pub trait IntoTableFuncArgs<'a> {
    fn into_args(self) -> JsonTableArgs<'a>;
}

impl<'a, T> IntoTableFuncArgs<'a> for Column<T, Sqlite> {
    #[inline(always)]
    fn into_args(self) -> JsonTableArgs<'a> {
        JsonTableArgs { base: self.into_arg(), path: None }
    }
}

impl<'a, T> IntoTableFuncArgs<'a> for (Column<T, Sqlite>, &'static str) {
    #[inline(always)]
    fn into_args(self) -> JsonTableArgs<'a> {
        JsonTableArgs {
            base: self.0.into_arg(),
            path: Some(Operand::from_value(Value::Text(Cow::Borrowed(self.1)))),
        }
    }
}
