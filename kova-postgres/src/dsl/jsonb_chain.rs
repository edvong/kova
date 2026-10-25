#![allow(non_snake_case)]

use crate::dsl::CanPgJsonb;
use crate::dsl::func::ExtFunction;
use crate::dsl::node::Node;
use crate::dsl::operator::JsonOp;
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, CanGroup, CanOrder, Dynamic, Operand, TypedNode,
};
use std::marker::PhantomData;

pub type JsonbChain<'a, T, X> = Node<InnerJsonbChain<'a, T, X>>;

pub(crate) fn JsonbChain<'a, T, X>(
    base: Operand<'a, Postgres>, op: JsonOp<'a>,
) -> JsonbChain<'a, T, X> {
    Node(InnerJsonbChain { base, op, _marker: PhantomData })
}

pub struct InnerJsonbChain<'a, T, X> {
    pub(crate) base: Operand<'a, Postgres>,
    pub(crate) op: JsonOp<'a>,
    pub(crate) _marker: PhantomData<(T, X)>,
}

impl<'a, T, X> TypedNode for InnerJsonbChain<'a, T, X> {
    type Output = Dynamic;
}

impl<'a, T, X> CanAlias for InnerJsonbChain<'a, T, X> {}
impl<'a, T, X> CanCompare for InnerJsonbChain<'a, T, X> {}
impl<'a, T, X> CanOrder for InnerJsonbChain<'a, T, X> {}
impl<'a, T, X> CanGroup for InnerJsonbChain<'a, T, X> {}
impl<'a, T, X> CanPgJsonb<X> for InnerJsonbChain<'a, T, X> {}

impl<'a, T, X> AstNode<'a, Postgres, X> for InnerJsonbChain<'a, T, X> {
    fn into_ast_operand(self) -> Operand<'a, Postgres> {
        Operand::from_function(Function::from(ExtFunction::JsonChain {
            base: Box::new(self.base),
            op: Box::new(self.op),
        }))
    }
}
