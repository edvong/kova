#![allow(non_snake_case, non_upper_case_globals)]

use crate::dsl::column::Column;
use crate::dsl::function::ExtSrfFunction;
use crate::dsl::function::srf::IntoSrfFunction;
use crate::dsl::function::srf::srf_node::{SrfBuildMarker, SrfNode, SrfSchemaMarker};
use kova_core::dsl::Dynamic;
use kova_core::types::IdentifierMeta;

pub type JsonbEach<T = ()> = SrfNode<T, JsonbEachMarker>;

pub const fn JsonbEach<T>(args: T) -> JsonbEach<T> {
    SrfNode::new(args)
}

pub struct JsonbEachSchema {
    pub key: Column<String, IdentifierMeta>,
    pub value: Column<Dynamic, IdentifierMeta, Dynamic>,
}

const JSONB_EACH_DEFAULT: JsonbEachSchema = JsonbEachSchema {
    key: Column::identifier("jsonb_each", "key"),
    value: Column::dynamic("jsonb_each", "value"),
};

impl JsonbEach<()> {
    pub const key: Column<String, IdentifierMeta> = JSONB_EACH_DEFAULT.key;
    pub const value: Column<Dynamic, IdentifierMeta, Dynamic> = JSONB_EACH_DEFAULT.value;
}

pub struct JsonbEachMarker;

impl SrfSchemaMarker for JsonbEachMarker {
    type Schema = JsonbEachSchema;
    fn schema() -> &'static Self::Schema {
        &JSONB_EACH_DEFAULT
    }
}

impl<'a, T, X> SrfBuildMarker<'a, T, X> for JsonbEachMarker
    where
        T: IntoSrfFunction<'a, X> {
    #[inline(always)]
    fn build_srf(args: T) -> ExtSrfFunction<'a> {
        ExtSrfFunction::JsonbEach(args.into_srf_function())
    }
}
