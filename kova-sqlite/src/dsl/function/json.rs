#![allow(non_snake_case)]

use crate::dsl::ExtFunction;
use crate::dsl::can::CanSqliteJson;
use crate::dsl::node::Node;
use crate::types::Sqlite;
use kova_core::dsl::func::{Function, Scalar, ScalarBuild, ScalarMarker};
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, CanGroup, CanMath, CanOrder, CanString, Dynamic,
    IntoArg, Operand, Projection, Static, TypedNode,
};
use std::marker::PhantomData;

// Json(X) Scalar Conversion Function (Reusing Core Scalar)
pub struct JsonMarker;

pub type Json<T> = Node<Scalar<JsonMarker, T>>;

pub const fn Json<T>(target: T) -> Json<T> {
    Node(Scalar::new(target))
}

impl<T: TypedNode> ScalarMarker<T> for JsonMarker {
    type Output = Static<String>;
}

impl<'a, T, X> ScalarBuild<'a, Sqlite, T, X> for JsonMarker
    where
        T: AstNode<'a, Sqlite, X>,
        T::Output: CanSqliteJson<X> {
    #[inline(always)]
    fn build_function(expr: T) -> Function<'a, Sqlite> {
        Function::from(ExtFunction::Json(Box::new(expr.into_ast_operand())))
    }
}

impl CanCompare for JsonMarker {}
impl CanOrder for JsonMarker {}
impl CanGroup for JsonMarker {}

// JsonObject & JsonbObject (Isomorphic Variable Parameter Object Constructor)
pub struct JsonObjectMarker;
pub struct JsonbObjectMarker;

pub type JsonObject<A, K, X> = Node<InnerJsonObject<JsonObjectMarker, A, K, X>>;
pub type JsonbObject<A, K, X> = Node<InnerJsonObject<JsonbObjectMarker, A, K, X>>;

pub const fn JsonObject<A, K, X>(args: A) -> JsonObject<A, K, X> {
    Node(InnerJsonObject { args, _marker: PhantomData })
}

pub const fn JsonbObject<A, K, X>(args: A) -> JsonbObject<A, K, X> {
    Node(InnerJsonObject { args, _marker: PhantomData })
}

pub trait ObjectBuildMarker {
    type Output;
    fn build_ext<'a>(items: Vec<Projection<'a, Sqlite, ()>>) -> ExtFunction<'a>;
}

impl ObjectBuildMarker for JsonObjectMarker {
    type Output = Static<String>;
    #[inline(always)]
    fn build_ext<'a>(items: Vec<Projection<'a, Sqlite, ()>>) -> ExtFunction<'a> {
        ExtFunction::JsonObject(items)
    }
}

impl ObjectBuildMarker for JsonbObjectMarker {
    type Output = Static<Vec<u8>>;
    #[inline(always)]
    fn build_ext<'a>(items: Vec<Projection<'a, Sqlite, ()>>) -> ExtFunction<'a> {
        ExtFunction::JsonbObject(items)
    }
}

pub struct InnerJsonObject<M, A, K, X> {
    args: A,
    _marker: PhantomData<(M, K, X)>,
}

impl<M: ObjectBuildMarker, A, K, X> TypedNode for InnerJsonObject<M, A, K, X> {
    type Output = M::Output;
}

impl<'a, M, A, K, X> AstNode<'a, Sqlite, X> for InnerJsonObject<M, A, K, X>
    where
        M: ObjectBuildMarker,
        A: IntoArg<Projection<'a, Sqlite, X>, K> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(M::build_ext(self.args.into_args_erased())))
    }
}

impl<M, A, K, X> CanAlias for InnerJsonObject<M, A, K, X> {}
impl<M, A, K, X> CanCompare for InnerJsonObject<M, A, K, X> {}
impl<A, K, X> CanString<X> for InnerJsonObject<JsonObjectMarker, A, K, X> {}
impl<M, A, K, X> CanSqliteJson<X> for InnerJsonObject<M, A, K, X> {}
// impl<A, K, X> CanSqliteString<X> for InnerJsonObject<JsonObjectMarker, A, K, X> {}

// JsonExtract & JsonbExtract (Isomorphic Dynamic Extraction of Nodes)
pub struct JsonExtractMarker;
pub struct JsonbExtractMarker;

pub type JsonExtract<'a, X> = Node<InnerJsonExtract<'a, JsonExtractMarker, X>>;
pub type JsonbExtract<'a, X> = Node<InnerJsonExtract<'a, JsonbExtractMarker, X>>;

pub trait ExtractBuildMarker {
    fn build_ext<'a>(
        base: Box<Operand<'a, Sqlite>>, path: Box<Operand<'a, Sqlite>>,
    ) -> ExtFunction<'a>;
}

impl ExtractBuildMarker for JsonExtractMarker {
    #[inline(always)]
    fn build_ext<'a>(
        base: Box<Operand<'a, Sqlite>>, path: Box<Operand<'a, Sqlite>>,
    ) -> ExtFunction<'a> {
        ExtFunction::JsonExtract(base, path)
    }
}

impl ExtractBuildMarker for JsonbExtractMarker {
    #[inline(always)]
    fn build_ext<'a>(
        base: Box<Operand<'a, Sqlite>>, path: Box<Operand<'a, Sqlite>>,
    ) -> ExtFunction<'a> {
        ExtFunction::JsonbExtract(base, path)
    }
}

pub struct InnerJsonExtract<'a, M, X> {
    pub(crate) base: Operand<'a, Sqlite>,
    pub(crate) path: Operand<'a, Sqlite>,
    pub(crate) _marker: PhantomData<(M, X)>,
}

impl<'a, M, X> TypedNode for InnerJsonExtract<'a, M, X> {
    type Output = Dynamic;
}

impl<'a, M: ExtractBuildMarker, X> AstNode<'a, Sqlite, X> for InnerJsonExtract<'a, M, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(M::build_ext(
            Box::new(self.base),
            Box::new(self.path),
        )))
    }
}

impl<'a, M, X> CanAlias for InnerJsonExtract<'a, M, X> {}
impl<'a, M, X> CanCompare for InnerJsonExtract<'a, M, X> {}
impl<'a, M, X> CanOrder for InnerJsonExtract<'a, M, X> {}
impl<'a, M, X> CanGroup for InnerJsonExtract<'a, M, X> {}
impl<'a, M, X> CanMath<X> for InnerJsonExtract<'a, M, X> {}
impl<'a, M, X> CanString<X> for InnerJsonExtract<'a, M, X> {}
impl<'a, M, X> CanSqliteJson<X> for InnerJsonExtract<'a, M, X> {}