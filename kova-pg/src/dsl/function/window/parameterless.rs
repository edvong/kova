#![allow(non_snake_case)]

use crate::dsl::ExtFunction;
use crate::dsl::node::Node;
use crate::types::Pg;
use kova_core::dsl::func::{Function, SetWindow, Window};
use kova_core::dsl::{AstNode, CanAlias, CanMath, CanOrder, CanWindow, Operand, Static, TypedNode};
use std::marker::PhantomData;

pub type RowNumber<'a, X> = Node<InnerWindowFunction<'a, RowNumberMarker, X>>;
pub type Rank<'a, X> = Node<InnerWindowFunction<'a, RankMarker, X>>;
pub type DenseRank<'a, X> = Node<InnerWindowFunction<'a, DenseRankMarker, X>>;
pub type PercentRank<'a, X> = Node<InnerWindowFunction<'a, PercentRankMarker, X>>;
pub type CumeDist<'a, X> = Node<InnerWindowFunction<'a, CumeDistMarker, X>>;

pub struct RowNumberMarker;
pub struct RankMarker;
pub struct DenseRankMarker;
pub struct PercentRankMarker;
pub struct CumeDistMarker;

pub const fn RowNumber<'a, X>() -> RowNumber<'a, X> {
    Node(InnerWindowFunction { window: None, _marker: PhantomData })
}

pub const fn Rank<'a, X>() -> Rank<'a, X> {
    Node(InnerWindowFunction { window: None, _marker: PhantomData })
}

pub const fn DenseRank<'a, X>() -> DenseRank<'a, X> {
    Node(InnerWindowFunction { window: None, _marker: PhantomData })
}

pub const fn PercentRank<'a, X>() -> PercentRank<'a, X> {
    Node(InnerWindowFunction { window: None, _marker: PhantomData })
}

pub const fn CumeDist<'a, X>() -> CumeDist<'a, X> {
    Node(InnerWindowFunction { window: None, _marker: PhantomData })
}

impl ParameterlessWindowMarker for RowNumberMarker {
    type Output = i64;
    fn ext_function<'a>() -> ExtFunction<'a> {
        ExtFunction::RowNumber
    }
}

impl ParameterlessWindowMarker for RankMarker {
    type Output = i64;
    fn ext_function<'a>() -> ExtFunction<'a> {
        ExtFunction::Rank
    }
}

impl ParameterlessWindowMarker for DenseRankMarker {
    type Output = i64;
    fn ext_function<'a>() -> ExtFunction<'a> {
        ExtFunction::DenseRank
    }
}

impl ParameterlessWindowMarker for PercentRankMarker {
    type Output = f64;
    fn ext_function<'a>() -> ExtFunction<'a> {
        ExtFunction::PercentRank
    }
}

impl ParameterlessWindowMarker for CumeDistMarker {
    type Output = f64;
    fn ext_function<'a>() -> ExtFunction<'a> {
        ExtFunction::CumeDist
    }
}

pub trait ParameterlessWindowMarker {
    // Tell me your output type (e.g. i64 or f64)
    type Output;
    // Tell me which underlying AST variant you correspond to
    fn ext_function<'a>() -> ExtFunction<'a>;
}

pub struct InnerWindowFunction<'a, M, X> {
    window: Option<Window<'a, Pg, ()>>,
    _marker: PhantomData<(M, X)>,
}

impl<'a, M: ParameterlessWindowMarker, X> TypedNode for InnerWindowFunction<'a, M, X> {
    type Output = Static<M::Output>;
}

impl<'a, M, X> CanAlias for InnerWindowFunction<'a, M, X> {}
impl<'a, M, X> CanOrder for InnerWindowFunction<'a, M, X> {}
impl<'a, M, X> CanMath for InnerWindowFunction<'a, M, X> {}
impl<'a, M, X> CanWindow for InnerWindowFunction<'a, M, X> {}

impl<'a, M, X> SetWindow<'a, Pg, X> for InnerWindowFunction<'a, M, X> {
    fn set_window(mut self, window: Window<'a, Pg, ()>) -> Self {
        self.window = Some(window);
        self
    }
}

impl<'a, M: ParameterlessWindowMarker, X> AstNode<'a, Pg, X> for InnerWindowFunction<'a, M, X> {
    fn into_ast_operand(self) -> Operand<'a, Pg> {
        let mut f = Function::from(M::ext_function());
        if let Some(window) = self.window {
            f = f.window(window);
        }
        Operand::from_function(f)
    }
}
