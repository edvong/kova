#![allow(non_snake_case)]

use crate::dsl::ExtFunction;
use crate::dsl::node::{CanPgJsonb, Node};
use crate::types::Pg;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, IntoArg, JsonbType, Operand, Projection, Static,
    TypedNode,
};
use std::marker::PhantomData;

pub type JsonbBuildObject<T, M, X> = Node<InnerJsonbBuildObject<T, M, X>>;

pub const fn JsonbBuildObject<A, M, X>(args: A) -> JsonbBuildObject<A, M, X> {
    Node(InnerJsonbBuildObject { args, _marker: PhantomData })
}

pub struct InnerJsonbBuildObject<A, M, X> {
    args: A,
    _marker: PhantomData<(M, X)>,
}

impl<A, M, X> TypedNode for InnerJsonbBuildObject<A, M, X> {
    type Output = Static<JsonbType>;
}

impl<T, M, X> CanAlias for InnerJsonbBuildObject<T, M, X> {}
impl<T, M, X> CanCompare for InnerJsonbBuildObject<T, M, X> {}
impl<T, M, X> CanPgJsonb for InnerJsonbBuildObject<T, M, X> {}

impl<'a, T, M, X> AstNode<'a, Pg, X> for InnerJsonbBuildObject<T, M, X>
    where
        T: IntoArg<Projection<'a, Pg, X>, M> {
    fn into_ast_operand(self) -> Operand<'a, Pg> {
        Operand::from_function(Function::from(ExtFunction::JsonbBuildObject(
            self.args.into_args_erased(),
        )))
    }
}
