#![allow(non_snake_case)]

use crate::dsl::func::ExtFunction;
use crate::dsl::node::Node;
use crate::types::Pg;
use kova_core::dsl::func::{Function, SetWindow, Window};
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, CanMath, CanOrder, CanWindow, IntoOperand, Operand, Static,
    TypedNode,
};
use std::marker::PhantomData;

pub type Ntile<'a, E, K, X> = Node<InnerNtile<'a, E, K, X>>;

pub const fn Ntile<'a, E, K, X>(buckets: E) -> Ntile<'a, E, K, X> {
    Node(InnerNtile { buckets, window: None, _marker: PhantomData })
}

pub struct InnerNtile<'a, E, K, X> {
    buckets: E,
    window: Option<Window<'a, Pg, ()>>,
    _marker: PhantomData<(K, X)>,
}

impl<'a, E, K, X> TypedNode for InnerNtile<'a, E, K, X> {
    type Output = Static<i32>;
}

impl<'a, E, K, X> CanCompare for InnerNtile<'a, E, K, X> {}
impl<'a, E, K, X> CanAlias for InnerNtile<'a, E, K, X> {}
impl<'a, E, K, X> CanOrder for InnerNtile<'a, E, K, X> {}
impl<'a, E, K, X> CanMath<X> for InnerNtile<'a, E, K, X> {}
impl<'a, E, K, X> CanWindow for InnerNtile<'a, E, K, X> {}

impl<'a, E, K, X> SetWindow<'a, Pg, X> for InnerNtile<'a, E, K, X> {
    fn set_window(mut self, window: Window<'a, Pg, ()>) -> Self {
        self.window = Some(window);
        self
    }
}

impl<'a, E, K, X> AstNode<'a, Pg, X> for InnerNtile<'a, E, K, X>
    where
        E: IntoOperand<'a, Static<i32>, Pg, K, X> {
    fn into_ast_operand(self) -> Operand<'a, Pg> {
        let mut f = Function::from(ExtFunction::Ntile(Box::new(self.buckets.into_operand())));
        if let Some(window) = self.window {
            f = f.window(window);
        }
        Operand::from_function(f)
    }
}
