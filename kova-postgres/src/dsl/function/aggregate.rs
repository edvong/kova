#![allow(non_snake_case)]

use crate::dsl::func::ExtFunction;
use crate::dsl::node::Node;
use crate::types::Postgres;
use kova_core::dsl::func::{Aggregate, AggregateBuild, AggregateMarker, Function};
use kova_core::dsl::{AstNode, IntoArrayOutput, JsonbType, Static, TypedNode};

// ==========================================
// 1. ArrayAgg
// ==========================================
pub struct ArrayAggMarker;

pub type ArrayAgg<'a, T, X> = Node<Aggregate<'a, Postgres, ArrayAggMarker, T, X>>;

pub const fn ArrayAgg<'a, T, X>(target: T) -> ArrayAgg<'a, T, X> {
    Node(Aggregate::new(target))
}

impl<T: TypedNode> AggregateMarker<T> for ArrayAggMarker
    where
        T::Output: IntoArrayOutput {
    type Output = <T::Output as IntoArrayOutput>::Output;
}

impl<'a, T, X> AggregateBuild<'a, Postgres, T, X> for ArrayAggMarker
    where
        T: AstNode<'a, Postgres, X>,
        T::Output: IntoArrayOutput {
    #[inline(always)]
    fn build_function(expr: T) -> Function<'a, Postgres> {
        Function::from(ExtFunction::ArrayAgg(Box::new(expr.into_ast_operand())))
    }
}

// ==========================================
// 2. JsonbAgg
// ==========================================
pub struct JsonbAggMarker;

pub type JsonbAgg<'a, T, X> = Node<Aggregate<'a, Postgres, JsonbAggMarker, T, X>>;

pub const fn JsonbAgg<'a, T, X>(target: T) -> JsonbAgg<'a, T, X> {
    Node(Aggregate::new(target))
}

impl<T: TypedNode> AggregateMarker<T> for JsonbAggMarker {
    type Output = Static<JsonbType>;
}

impl<'a, T, X> AggregateBuild<'a, Postgres, T, X> for JsonbAggMarker
    where
        T: AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn build_function(expr: T) -> Function<'a, Postgres> {
        Function::from(ExtFunction::JsonbAgg(Box::new(expr.into_ast_operand())))
    }
}
