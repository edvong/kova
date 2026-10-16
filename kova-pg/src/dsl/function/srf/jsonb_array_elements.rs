#![allow(non_snake_case, non_upper_case_globals)]

use crate::dsl::column::Column;
use crate::dsl::function::ExtSrfFunction;
use crate::dsl::function::srf::IntoSrfFunction;
use crate::dsl::function::srf::srf_node::{SrfBuildMarker, SrfNode, SrfSchemaMarker};
use kova_core::dsl::Dynamic;
use kova_core::types::IdentifierMeta;

pub type JsonbArrayElements<T = ()> = SrfNode<T, JsonbArrayElementsMarker>;

pub const fn JsonbArrayElements<T>(args: T) -> JsonbArrayElements<T> {
    SrfNode::new(args)
}

pub struct JsonbArrayElementsSchema {
    pub value: Column<Dynamic, IdentifierMeta, Dynamic>,
}

const JSONB_ARRAY_ELEMENTS_DEFAULT: JsonbArrayElementsSchema =
    JsonbArrayElementsSchema { value: Column::dynamic("jsonb_array_elements", "value") };

impl JsonbArrayElements<()> {
    pub const value: Column<Dynamic, IdentifierMeta, Dynamic> = JSONB_ARRAY_ELEMENTS_DEFAULT.value;
}

pub struct JsonbArrayElementsMarker;

impl SrfSchemaMarker for JsonbArrayElementsMarker {
    type Schema = JsonbArrayElementsSchema;
    fn schema() -> &'static Self::Schema {
        &JSONB_ARRAY_ELEMENTS_DEFAULT
    }
}

impl<'a, T, X> SrfBuildMarker<'a, T, X> for JsonbArrayElementsMarker
    where
        T: IntoSrfFunction<'a, X> {
    #[inline(always)]
    fn build_srf(args: T) -> ExtSrfFunction<'a> {
        ExtSrfFunction::JsonbArrayElements(args.into_srf_function())
    }
}
