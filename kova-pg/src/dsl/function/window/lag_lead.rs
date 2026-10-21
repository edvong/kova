#![allow(non_snake_case)]

use crate::dsl::can::{CanPgArray, CanPgJsonb};
use crate::dsl::func::ExtFunction;
use crate::dsl::node::Node;
use crate::types::Pg;
use kova_core::dsl::func::{Function, SetWindow, Window};
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, CanMath, CanOrder, CanString, CanWindow, Close, IntoOperand,
    Open, Operand, Static, TypedNode,
};
use std::marker::PhantomData;

pub type Lag<'a, E, O, D, KO, KD, X> = Node<InnerOffsetFunc<'a, LagMarker, E, O, D, KO, KD, X>>;
pub type Lead<'a, E, O, D, KO, KD, X> = Node<InnerOffsetFunc<'a, LeadMarker, E, O, D, KO, KD, X>>;

pub const fn Lag<'a, E, X>(expr: E) -> Lag<'a, E, i32, E, Open, Close, X> {
    Node(InnerOffsetFunc { expr, offset: None, default: None, window: None, _marker: PhantomData })
}

pub const fn Lead<'a, E, X>(expr: E) -> Lead<'a, E, i32, E, Open, Close, X> {
    Node(InnerOffsetFunc { expr, offset: None, default: None, window: None, _marker: PhantomData })
}

pub struct LagMarker;
pub struct LeadMarker;

pub trait OffsetWindowMarker {
    fn ext_function<'a>(
        expr: Box<Operand<'a, Pg>>, offset: Option<Box<Operand<'a, Pg>>>,
        default: Option<Box<Operand<'a, Pg>>>,
    ) -> ExtFunction<'a>;
}

impl OffsetWindowMarker for LagMarker {
    fn ext_function<'a>(
        expr: Box<Operand<'a, Pg>>, offset: Option<Box<Operand<'a, Pg>>>,
        default: Option<Box<Operand<'a, Pg>>>,
    ) -> ExtFunction<'a> {
        ExtFunction::Lag { expr, offset, default }
    }
}

impl OffsetWindowMarker for LeadMarker {
    fn ext_function<'a>(
        expr: Box<Operand<'a, Pg>>, offset: Option<Box<Operand<'a, Pg>>>,
        default: Option<Box<Operand<'a, Pg>>>,
    ) -> ExtFunction<'a> {
        ExtFunction::Lead { expr, offset, default }
    }
}

pub struct InnerOffsetFunc<'a, M, E, O, D, KO, KD, X> {
    expr: E,
    offset: Option<O>,
    default: Option<D>,
    window: Option<Window<'a, Pg, ()>>,
    _marker: PhantomData<(M, KO, KD, X)>,
}

impl<'a, M, E: TypedNode, O, D, KO, KD, X> TypedNode
    for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X> {
    type Output = E::Output;
}

impl<'a, M, E, O, D, KO, KD, X> CanAlias for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X> {}
impl<'a, M, E, O, D, KO, KD, X> CanCompare for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X> {}
impl<'a, M, E, O, D, KO, KD, X> CanOrder for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X> {}
impl<'a, M, E, O, D, KO, KD, X> CanWindow for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X> {}

impl<'a, M, E, O, D, KO, KD, X> CanMath<X> for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanMath<X> {
}

impl<'a, M, E, O, D, KO, KD, X> CanString<X> for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanString<X> {
}

impl<'a, M, E, O, D, KO, KD, X> CanPgArray<X> for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanPgArray<X> {
}

impl<'a, M, E, O, D, KO, KD, X> CanPgJsonb<X> for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanPgJsonb<X> {
}

impl<'a, M, E, O, D, KO, KD, X> SetWindow<'a, Pg, X>
    for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X> {
    fn set_window(mut self, window: Window<'a, Pg, ()>) -> Self {
        self.window = Some(window);
        self
    }
}

impl<'a, M, E, O, D, KO, KD, X> Node<InnerOffsetFunc<'a, M, E, O, D, KO, KD, X>> {
    pub fn offset<O2, KO2>(self, offset: O2) -> Node<InnerOffsetFunc<'a, M, E, O2, D, KO2, KD, X>> {
        Node(InnerOffsetFunc {
            expr: self.0.expr,
            offset: Some(offset),
            default: self.0.default,
            window: self.0.window,
            _marker: PhantomData,
        })
    }

    pub fn default<D2, KD2>(
        self, default: D2,
    ) -> Node<InnerOffsetFunc<'a, M, E, O, D2, KO, KD2, X>> {
        Node(InnerOffsetFunc {
            expr: self.0.expr,
            offset: self.0.offset,
            default: Some(default),
            window: self.0.window,
            _marker: PhantomData,
        })
    }
}

impl<'a, M, E, O, D, KO, KD, X> AstNode<'a, Pg, X> for InnerOffsetFunc<'a, M, E, O, D, KO, KD, X>
    where
        M: OffsetWindowMarker,
        E: AstNode<'a, Pg, X>,
        O: IntoOperand<'a, Static<i32>, Pg, KO, X>,
        D: IntoOperand<'a, E::Output, Pg, KD, X> {
    fn into_ast_operand(self) -> Operand<'a, Pg> {
        let core_ast = M::ext_function(
            Box::new(self.expr.into_ast_operand()),
            self.offset.map(|o| Box::new(o.into_operand())),
            self.default.map(|d| Box::new(d.into_operand())),
        );
        let mut f = Function::from(core_ast);
        if let Some(window) = self.window {
            f = f.window(window);
        }
        Operand::from_function(f)
    }
}
