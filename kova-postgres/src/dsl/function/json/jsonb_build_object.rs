#![allow(non_snake_case)]

use crate::dsl::CanPgJsonb;
use crate::dsl::func::ExtFunction;
use crate::dsl::node::Node;
use crate::types::Postgres;
use kova_kernel::dsl::func::Function;
use kova_kernel::dsl::{
    AstNode, CanAlias, CanCompare, IntoArg, JsonbType, Operand, Projection, Static, TypedNode,
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
impl<T, M, X> CanPgJsonb<X> for InnerJsonbBuildObject<T, M, X> {}

impl<'a, T, M, X> AstNode<'a, Postgres, X> for InnerJsonbBuildObject<T, M, X>
    where
        T: IntoArg<Projection<'a, Postgres, X>, M> {
    fn into_ast_operand(self) -> Operand<'a, Postgres> {
        Operand::from_function(Function::from(ExtFunction::JsonbBuildObject(
            self.args.into_args_erased(),
        )))
    }
}
