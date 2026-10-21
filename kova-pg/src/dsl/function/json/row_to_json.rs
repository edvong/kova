#![allow(non_snake_case)]

use crate::dsl::func::ExtFunction;
use crate::dsl::node::Node;
use crate::types::Pg;
use kova_core::dsl::func::{Function, Scalar, ScalarBuild, ScalarMarker};
use kova_core::dsl::{AstNode, CanCompare, JsonType, Static, TypedNode};

pub struct RowToJsonMarker;

pub type RowToJson<T> = Node<Scalar<RowToJsonMarker, T>>;

pub const fn RowToJson<T>(target: T) -> RowToJson<T> {
    Node(Scalar::new(target))
}

impl<T: TypedNode> ScalarMarker<T> for RowToJsonMarker {
    type Output = Static<JsonType>;
}

impl<'a, T, X> ScalarBuild<'a, Pg, T, X> for RowToJsonMarker
    where
        T: AstNode<'a, Pg, X> {
    #[inline(always)]
    fn build_function(expr: T) -> Function<'a, Pg> {
        Function::from(ExtFunction::RowToJson(Box::new(expr.into_ast_operand())))
    }
}

impl CanCompare for RowToJsonMarker {}
