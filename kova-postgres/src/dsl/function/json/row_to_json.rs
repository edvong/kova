#![allow(non_snake_case)]

use crate::dsl::func::ExtFunction;
use crate::dsl::node::Node;
use crate::types::Postgres;
use kova_core::dsl::func::{Function, Scalar, ScalarBuild, ScalarMarker};
use kova_core::dsl::{AstNode, CanCompare, JsonType, Static, Table};

pub struct RowToJsonMarker;

pub type RowToJson<T> = Node<Scalar<RowToJsonMarker, T>>;

pub const fn RowToJson<T>(target: T) -> RowToJson<T> {
    Node(Scalar::new(target))
}

impl<T> ScalarMarker<Table<T>> for RowToJsonMarker {
    type Output = Static<JsonType>;
}

impl<'a, T, X> ScalarBuild<'a, Postgres, Table<T>, X> for RowToJsonMarker {
    #[inline(always)]
    fn build_function(expr: Table<T>) -> Function<'a, Postgres> {
        Function::from(ExtFunction::RowToJson(
            Box::new(<Table<T> as AstNode<'_, Postgres, X>>::into_ast_operand(expr)))
        )
    }
}

impl CanCompare for RowToJsonMarker {}
