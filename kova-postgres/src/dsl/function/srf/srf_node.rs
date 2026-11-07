use crate::dsl::function::srf::ExtSrfFunction;
use crate::types::Postgres;
use kova_kernel::dsl::{ArgAst, FromItem, IntoArg};
use std::any::TypeId;
use std::marker::PhantomData;
use std::ops::Deref;

pub struct Untagged;
pub struct Tagged;

pub struct SrfNode<A, M, Tag = Untagged> {
    pub args: A,
    pub(crate) tag: Option<TypeId>,
    _marker: PhantomData<(M, Tag)>,
}

impl<A: Copy, M, Tag> Copy for SrfNode<A, M, Tag> {}
impl<A: Copy, M, Tag> Clone for SrfNode<A, M, Tag> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<A, M> SrfNode<A, M, Untagged> {
    #[inline(always)]
    pub const fn new(args: A) -> Self {
        Self { args, tag: None, _marker: PhantomData }
    }

    #[inline(always)]
    pub fn tag<Marker: 'static>(self) -> SrfNode<A, M, Tagged> {
        SrfNode { args: self.args, tag: Some(TypeId::of::<Marker>()), _marker: PhantomData }
    }
}

pub trait SrfSchemaMarker {
    type Schema: 'static;
    fn schema() -> &'static Self::Schema;
}

impl<A, M: SrfSchemaMarker, Tag> Deref for SrfNode<A, M, Tag> {
    type Target = M::Schema;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        M::schema()
    }
}

pub trait SrfBuildMarker<'a, A, X> {
    fn build_srf(args: A) -> ExtSrfFunction<'a>;
}

impl<'a, A, M, Tag, X> IntoArg<FromItem<'a, Postgres, X>, ArgAst> for SrfNode<A, M, Tag>
    where
        M: SrfBuildMarker<'a, A, X> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, Postgres, X> {
        FromItem::from_function(M::build_srf(self.args), self.tag)
    }
}

impl<'a, 'b, A, M, Tag, X> IntoArg<FromItem<'a, Postgres, X>, ArgAst> for &'b SrfNode<A, M, Tag>
    where
        A: Copy,
        M: SrfBuildMarker<'a, A, X> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, Postgres, X> {
        FromItem::from_function(M::build_srf(self.args), self.tag)
    }
}
