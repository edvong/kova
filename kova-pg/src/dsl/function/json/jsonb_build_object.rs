#![allow(non_snake_case)]

use crate::dsl::ExtFunction;
use crate::dsl::node::{CanPgJsonb, Node};
use crate::types::Pg;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, EraseX, IntoArgs, JsonbType, Operand, Projection, Static, TypedNode,
};
use std::marker::PhantomData;

pub type JsonbBuildObject<T, X> = Node<InnerJsonbBuildObject<T, X>>;

pub const fn JsonbBuildObject<'a, A, X>(args: A) -> JsonbBuildObject<A, X> {
    Node(InnerJsonbBuildObject { args, _marker: PhantomData })
}

pub struct InnerJsonbBuildObject<A, X> {
    args: A,
    _marker: PhantomData<X>,
}

impl<A, X> TypedNode for InnerJsonbBuildObject<A, X> {
    type Output = Static<JsonbType>;
}

impl<T, X> CanAlias for InnerJsonbBuildObject<T, X> {}
impl<T, X> CanCompare for InnerJsonbBuildObject<T, X> {}
impl<T, X> CanPgJsonb for InnerJsonbBuildObject<T, X> {}

impl<'a, T, X> AstNode<'a, Pg, X> for InnerJsonbBuildObject<T, X>
    where
        T: IntoArgs<Projection<'a, Pg, X>> {
    fn into_ast_operand(self) -> Operand<'a, Pg> {
        Operand::from_function(Function::from(ExtFunction::JsonbBuildObject(
            self.args.into_mapped_args(|p| p.erase_x()),
        )))
    }
}
