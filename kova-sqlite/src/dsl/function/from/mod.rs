use crate::types::{Sqlite, Value};
use kova_core::dsl::{Column, IntoArg, Operand};
use std::borrow::Cow;

mod json_each;
#[rustfmt::skip]
mod json_tree;

pub use json_each::{JsonEach, JsonEachSchema, JSON_EACH_DEFAULT};
pub use json_tree::{JsonTree, JsonTreeSchema, JSON_TREE_DEFAULT};

pub enum ExtTableFunction<'a> {
    JsonEach(Operand<'a, Sqlite>, Option<Operand<'a, Sqlite>>),
    JsonTree(Operand<'a, Sqlite>, Option<Operand<'a, Sqlite>>),
}

pub struct JsonTableArgs<'a> {
    pub base: Operand<'a, Sqlite>,
    pub path: Option<Operand<'a, Sqlite>>,
}

pub trait IntoJsonTableArgs<'a> {
    fn into_args(self) -> JsonTableArgs<'a>;
}

impl<'a, T> IntoJsonTableArgs<'a> for Column<'a, T, Sqlite> {
    #[inline(always)]
    fn into_args(self) -> JsonTableArgs<'a> {
        JsonTableArgs { base: self.into_arg(), path: None }
    }
}

impl<'a, T> IntoJsonTableArgs<'a> for (Column<'a, T, Sqlite>, &'static str) {
    #[inline(always)]
    fn into_args(self) -> JsonTableArgs<'a> {
        JsonTableArgs {
            base: self.0.into_arg(),
            path: Some(Operand::from_value(Value::Text(Cow::Borrowed(self.1)))),
        }
    }
}
