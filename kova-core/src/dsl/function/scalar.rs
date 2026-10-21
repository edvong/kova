use crate::dsl::can::CanConflict;
use crate::dsl::function::{Function, StandardFunction};
use crate::dsl::{
    AstNode, CanAlias, CanCompare, CanGroup, CanMath, CanOrder, CanString, Operand, Static,
    TypedNode,
};
use crate::types::Database;
use std::marker::PhantomData;

pub type Abs<E> = Scalar<AbsMarker, E>;
pub type Length<E> = Scalar<LengthMarker, E>;
pub type Lower<E> = Scalar<LowerMarker, E>;
pub type Upper<E> = Scalar<UpperMarker, E>;

pub struct Scalar<M, E>(pub E, PhantomData<M>);

impl<M, E> Scalar<M, E> {
    #[inline(always)]
    pub const fn new(expr: E) -> Self {
        Self(expr, PhantomData)
    }
}

pub trait ScalarMarker<E> {
    type Output;
}

pub trait ScalarBuild<'a, DB: Database, E, X>: ScalarMarker<E> {
    fn build_function(expr: E) -> Function<'a, DB>;
}

impl<M, E> TypedNode for Scalar<M, E>
    where
        M: ScalarMarker<E> {
    type Output = M::Output;
}

impl<'a, DB: Database, M, E, X> AstNode<'a, DB, X> for Scalar<M, E>
    where
        M: ScalarBuild<'a, DB, E, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, DB> {
        Operand::from_function(M::build_function(self.0))
    }
}

impl<M, E> CanAlias for Scalar<M, E> {}
impl<M: CanCompare, E> CanCompare for Scalar<M, E> {}
impl<M: CanOrder, E> CanOrder for Scalar<M, E> {}
impl<M: CanGroup, E: CanGroup> CanGroup for Scalar<M, E> {}
impl<M, E: CanConflict> CanConflict for Scalar<M, E> {}

impl<M, E, X> CanMath<X> for Scalar<M, E>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanMath<X> {
}

impl<M, E, X> CanString<X> for Scalar<M, E>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanString<X> {
}

macro_rules! impl_std_scalar {
    ($marker:ident, $variant:ident, $input_bound:ident => $output:ty) => {
        pub struct $marker;

        impl<E: TypedNode> ScalarMarker<E> for $marker {
            type Output = $output;
        }

        impl<'a, DB: Database, E, X> ScalarBuild<'a, DB, E, X> for $marker
            where
                E: AstNode<'a, DB, X>,
                E::Output: $input_bound<X> {
            #[inline(always)]
            fn build_function(expr: E) -> Function<'a, DB> {
                Function::from(StandardFunction::$variant(Box::new(expr.into_ast_operand())))
            }
        }

        impl CanCompare for $marker {}
        impl CanOrder for $marker {}
        impl CanGroup for $marker {}
    };
}

impl_std_scalar!(AbsMarker, Abs, CanMath => E::Output);
impl_std_scalar!(LengthMarker, Length, CanString => Static<i64>);
impl_std_scalar!(LowerMarker, Lower, CanString => Static<String>);
impl_std_scalar!(UpperMarker, Upper, CanString => Static<String>);
