use crate::dsl::function::srf::ExtSrfFunction;
use crate::types::Sqlite;
use kova_core::dsl::{ArgAst, FromItem, IntoArg};
use std::marker::PhantomData;
use std::ops::Deref;

pub struct SrfNode<A, M> {
    pub args: A,
    _marker: PhantomData<M>,
}

impl<A: Copy, M> Copy for SrfNode<A, M> {}
impl<A: Copy, M> Clone for SrfNode<A, M> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<A, M> SrfNode<A, M> {
    pub const fn new(args: A) -> Self {
        Self { args, _marker: PhantomData }
    }
}

pub trait SrfSchemaMarker {
    type Schema: 'static;
    fn schema() -> &'static Self::Schema;
}

impl<A, M: SrfSchemaMarker> Deref for SrfNode<A, M> {
    type Target = M::Schema;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        M::schema()
    }
}

pub trait SrfBuildMarker<'a, A, X> {
    fn build_srf(args: A) -> ExtSrfFunction<'a>;
}

impl<'a, A, M, X> IntoArg<FromItem<'a, Sqlite, X>, ArgAst> for SrfNode<A, M>
    where
        M: SrfBuildMarker<'a, A, X> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, Sqlite, X> {
        FromItem::from_function(M::build_srf(self.args), None)
    }
}

impl<'a, 'b, A, M, X> IntoArg<FromItem<'a, Sqlite, X>, ArgAst> for &'b SrfNode<A, M>
    where
        A: Copy,
        M: SrfBuildMarker<'a, A, X> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, Sqlite, X> {
        FromItem::from_function(M::build_srf(self.args), None)
    }
}
