#![allow(non_snake_case)]

use crate::dsl::ExtFunction;
use crate::dsl::node::{CanPgJsonb, Node};
use crate::dsl::operator::JsonOp;
use crate::types::{Pg, Postgres};
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, CanGroup, CanOrder, Dynamic, Operand, TypedNode,
};
use std::marker::PhantomData;

pub type JsonbChain<'a, T, X> = Node<InnerJsonbChain<'a, T, X>>;

pub(crate) fn JsonbChain<'a, T, X>(
    base: Operand<'a, Postgres>, ops: Vec<JsonOp<'a>>,
) -> JsonbChain<'a, T, X> {
    Node(InnerJsonbChain { base, ops, _marker: PhantomData })
}

pub struct InnerJsonbChain<'a, T, X> {
    pub(crate) base: Operand<'a, Postgres>,
    pub(crate) ops: Vec<JsonOp<'a>>,
    pub(crate) _marker: PhantomData<(T, X)>,
}

impl<'a, T, X> TypedNode for InnerJsonbChain<'a, T, X> {
    type Output = Dynamic;
}

impl<'a, T, X> CanAlias for InnerJsonbChain<'a, T, X> {}
impl<'a, T, X> CanCompare for InnerJsonbChain<'a, T, X> {}
impl<'a, T, X> CanOrder for InnerJsonbChain<'a, T, X> {}
impl<'a, T, X> CanGroup for InnerJsonbChain<'a, T, X> {}
impl<'a, T, X> CanPgJsonb for InnerJsonbChain<'a, T, X> {}

impl<'a, T, X> AstNode<'a, Pg, X> for InnerJsonbChain<'a, T, X> {
    fn into_ast_operand(self) -> Operand<'a, Pg> {
        Operand::from_function(Function::from(ExtFunction::JsonChain {
            base: Box::new(self.base),
            ops: self.ops,
        }))
    }
}
