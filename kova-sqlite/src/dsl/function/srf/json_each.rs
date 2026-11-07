#![allow(non_snake_case, non_upper_case_globals)]

use crate::dsl::column::Column;
use crate::dsl::function::srf::srf_node::{SrfBuildMarker, SrfNode, SrfSchemaMarker};
use crate::dsl::function::srf::{ExtSrfFunction, IntoSrfFunction};
use kova_kernel::dsl::Dynamic;
use kova_kernel::types::IdentifierMeta;

pub type JsonEach<T = ()> = SrfNode<T, JsonEachMarker>;

pub const fn JsonEach<T>(args: T) -> JsonEach<T> {
    SrfNode::new(args)
}

pub struct JsonEachSchema {
    pub key: Column<String, IdentifierMeta>,
    pub value: Column<Dynamic, IdentifierMeta, Dynamic>,
    pub type_: Column<String, IdentifierMeta>,
    pub atom: Column<Dynamic, IdentifierMeta, Dynamic>,
    pub id: Column<i64, IdentifierMeta>,
    pub parent: Column<i64, IdentifierMeta>,
    pub fullkey: Column<String, IdentifierMeta>,
    pub path: Column<String, IdentifierMeta>,
}

pub const JSON_EACH_DEFAULT: JsonEachSchema = JsonEachSchema {
    key: Column::identifier("json_each", "key"),
    value: Column::dynamic("json_each", "value"),
    type_: Column::identifier("json_each", "type"),
    atom: Column::dynamic("json_each", "atom"),
    id: Column::identifier("json_each", "id"),
    parent: Column::identifier("json_each", "parent"),
    fullkey: Column::identifier("json_each", "fullkey"),
    path: Column::identifier("json_each", "path"),
};

impl JsonEach<()> {
    pub const key: Column<String, IdentifierMeta> = JSON_EACH_DEFAULT.key;
    pub const value: Column<Dynamic, IdentifierMeta, Dynamic> = JSON_EACH_DEFAULT.value;
    pub const type_: Column<String, IdentifierMeta> = JSON_EACH_DEFAULT.type_;
    pub const atom: Column<Dynamic, IdentifierMeta, Dynamic> = JSON_EACH_DEFAULT.atom;
    pub const id: Column<i64, IdentifierMeta> = JSON_EACH_DEFAULT.id;
    pub const parent: Column<i64, IdentifierMeta> = JSON_EACH_DEFAULT.parent;
    pub const fullkey: Column<String, IdentifierMeta> = JSON_EACH_DEFAULT.fullkey;
    pub const path: Column<String, IdentifierMeta> = JSON_EACH_DEFAULT.path;
}

pub struct JsonEachMarker;

impl SrfSchemaMarker for JsonEachMarker {
    type Schema = JsonEachSchema;

    #[inline(always)]
    fn schema() -> &'static Self::Schema {
        &JSON_EACH_DEFAULT
    }
}

impl<'a, T, X> SrfBuildMarker<'a, T, X> for JsonEachMarker
where
    T: IntoSrfFunction<'a, X>,
{
    #[inline(always)]
    fn build_srf(args: T) -> ExtSrfFunction<'a> {
        let parsed = args.into_srf_function();
        ExtSrfFunction::JsonEach(parsed.base, parsed.path)
    }
}
