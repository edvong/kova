#![allow(non_snake_case, non_upper_case_globals)]

use crate::dsl::column::Column;
use crate::dsl::function::ExtSrfFunction;
use crate::dsl::function::srf::IntoArraySrf;
use crate::dsl::function::srf::srf_node::{SrfBuildMarker, SrfNode, SrfSchemaMarker};
use kova_core::dsl::Dynamic;
use kova_core::types::IdentifierMeta;

pub type Unnest<T = ()> = SrfNode<T, UnnestMarker>;

pub const fn Unnest<T>(args: T) -> Unnest<T> {
    SrfNode::new(args)
}

pub struct UnnestSchema {
    pub unnest: Column<Dynamic, IdentifierMeta, Dynamic>,
}

const UNNEST_DEFAULT: UnnestSchema = UnnestSchema { unnest: Column::dynamic("unnest", "unnest") };

impl Unnest<()> {
    pub const unnest: Column<Dynamic, IdentifierMeta, Dynamic> = UNNEST_DEFAULT.unnest;
}

pub struct UnnestMarker;

impl SrfSchemaMarker for UnnestMarker {
    type Schema = UnnestSchema;
    fn schema() -> &'static Self::Schema {
        &UNNEST_DEFAULT
    }
}

impl<'a, T, X> SrfBuildMarker<'a, T, X> for UnnestMarker
    where
        T: IntoArraySrf<'a, X> {
    #[inline(always)]
    fn build_srf(args: T) -> ExtSrfFunction<'a> {
        ExtSrfFunction::Unnest(args.into_array_srf())
    }
}
