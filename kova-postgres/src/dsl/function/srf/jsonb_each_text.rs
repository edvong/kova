#![allow(non_snake_case, non_upper_case_globals)]

use crate::dsl::column::Column;
use crate::dsl::function::ExtSrfFunction;
use crate::dsl::function::srf::IntoSrfFunction;
use crate::dsl::function::srf::srf_node::{SrfBuildMarker, SrfNode, SrfSchemaMarker};
use kova_kernel::types::IdentifierMeta;

pub type JsonbEachText<T = ()> = SrfNode<T, JsonbEachTextMarker>;

pub const fn JsonbEachText<T>(args: T) -> JsonbEachText<T> {
    SrfNode::new(args)
}

pub struct JsonbEachTextSchema {
    pub key: Column<String, IdentifierMeta>,
    pub value: Column<String, IdentifierMeta>,
}

const JSONB_EACH_TEXT_DEFAULT: JsonbEachTextSchema = JsonbEachTextSchema {
    key: Column::identifier("jsonb_each_text", "key"),
    value: Column::identifier("jsonb_each_text", "value"),
};

impl JsonbEachText<()> {
    pub const key: Column<String, IdentifierMeta> = JSONB_EACH_TEXT_DEFAULT.key;
    pub const value: Column<String, IdentifierMeta> = JSONB_EACH_TEXT_DEFAULT.value;
}

pub struct JsonbEachTextMarker;

impl SrfSchemaMarker for JsonbEachTextMarker {
    type Schema = JsonbEachTextSchema;
    fn schema() -> &'static Self::Schema {
        &JSONB_EACH_TEXT_DEFAULT
    }
}

impl<'a, T, X> SrfBuildMarker<'a, T, X> for JsonbEachTextMarker
where
    T: IntoSrfFunction<'a, X>,
{
    #[inline(always)]
    fn build_srf(args: T) -> ExtSrfFunction<'a> {
        ExtSrfFunction::JsonbEachText(args.into_srf_function())
    }
}
