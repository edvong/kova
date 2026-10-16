use crate::dsl::node::{CanPgArray, CanPgJsonb, CanPgString};
use crate::types::Pg;
use kova_core::dsl::func::{Aggregate, LowerMarker, Scalar, UpperMarker};

impl<'a, M: CanPgString, E, X> CanPgString for Aggregate<'a, Pg, M, E, X> {}
impl<'a, M: CanPgArray, E, X> CanPgArray for Aggregate<'a, Pg, M, E, X> {}
impl<'a, M: CanPgJsonb, E, X> CanPgJsonb for Aggregate<'a, Pg, M, E, X> {}

impl<M: CanPgString, E> CanPgString for Scalar<M, E> {}
impl<M: CanPgArray, E> CanPgArray for Scalar<M, E> {}
impl<M: CanPgJsonb, E> CanPgJsonb for Scalar<M, E> {}

impl CanPgString for LowerMarker {}
impl CanPgString for UpperMarker {}
