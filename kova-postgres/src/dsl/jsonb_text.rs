#![allow(non_snake_case)]

use crate::dsl::function::ExtFunction;
use crate::dsl::node::Node;
use crate::dsl::operator::JsonOp;
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, CanGroup, CanOrder, CanString, Operand, Static, TypedNode,
};
use std::marker::PhantomData;

pub type JsonbText<'a, X> = Node<InnerJsonbText<'a, X>>;

pub(crate) fn JsonbText<'a, X>(base: Operand<'a, Postgres>, op: JsonOp<'a>) -> JsonbText<'a, X> {
    Node(InnerJsonbText { base, op, _marker: PhantomData })
}

pub struct InnerJsonbText<'a, X> {
    pub(crate) base: Operand<'a, Postgres>,
    pub(crate) op: JsonOp<'a>,
    pub(crate) _marker: PhantomData<X>,
}

impl<'a, X> TypedNode for InnerJsonbText<'a, X> {
    type Output = Static<String>;
}

impl<'a, X> CanAlias for InnerJsonbText<'a, X> {}
impl<'a, X> CanCompare for InnerJsonbText<'a, X> {}
impl<'a, X> CanOrder for InnerJsonbText<'a, X> {}
impl<'a, X> CanGroup for InnerJsonbText<'a, X> {}
impl<'a, X> CanString<X> for InnerJsonbText<'a, X> {}

impl<'a, X> AstNode<'a, Postgres, X> for InnerJsonbText<'a, X> {
    fn into_ast_operand(self) -> Operand<'a, Postgres> {
        Operand::from_function(Function::from(ExtFunction::JsonChain {
            base: Box::new(self.base),
            op: Box::new(self.op),
        }))
    }
}
