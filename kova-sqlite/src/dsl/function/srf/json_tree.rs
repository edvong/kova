#![allow(non_snake_case, non_upper_case_globals)]

use crate::dsl::column::Column;
use crate::dsl::function::srf::srf_node::{SrfBuildMarker, SrfNode, SrfSchemaMarker};
use crate::dsl::function::srf::{ExtSrfFunction, IntoSrfFunction};
use kova_core::dsl::Dynamic;
use kova_core::types::IdentifierMeta;

pub type JsonTree<T = ()> = SrfNode<T, JsonTreeMarker>;

pub const fn JsonTree<T>(args: T) -> JsonTree<T> {
    SrfNode::new(args)
}

pub struct JsonTreeSchema {
    pub key: Column<String, IdentifierMeta>,
    pub value: Column<Dynamic, IdentifierMeta, Dynamic>,
    pub type_: Column<String, IdentifierMeta>,
    pub atom: Column<Dynamic, IdentifierMeta, Dynamic>,
    pub id: Column<i64, IdentifierMeta>,
    pub parent: Column<i64, IdentifierMeta>,
    pub fullkey: Column<String, IdentifierMeta>,
    pub path: Column<String, IdentifierMeta>,
}

pub const JSON_TREE_DEFAULT: JsonTreeSchema = JsonTreeSchema {
    key: Column::identifier("json_tree", "key"),
    value: Column::dynamic("json_tree", "value"),
    type_: Column::identifier("json_tree", "type"),
    atom: Column::dynamic("json_tree", "atom"),
    id: Column::identifier("json_tree", "id"),
    parent: Column::identifier("json_tree", "parent"),
    fullkey: Column::identifier("json_tree", "fullkey"),
    path: Column::identifier("json_tree", "path"),
};

impl JsonTree<()> {
    pub const key: Column<String, IdentifierMeta> = JSON_TREE_DEFAULT.key;
    pub const value: Column<Dynamic, IdentifierMeta, Dynamic> = JSON_TREE_DEFAULT.value;
    pub const type_: Column<String, IdentifierMeta> = JSON_TREE_DEFAULT.type_;
    pub const atom: Column<Dynamic, IdentifierMeta, Dynamic> = JSON_TREE_DEFAULT.atom;
    pub const id: Column<i64, IdentifierMeta> = JSON_TREE_DEFAULT.id;
    pub const parent: Column<i64, IdentifierMeta> = JSON_TREE_DEFAULT.parent;
    pub const fullkey: Column<String, IdentifierMeta> = JSON_TREE_DEFAULT.fullkey;
    pub const path: Column<String, IdentifierMeta> = JSON_TREE_DEFAULT.path;
}

pub struct JsonTreeMarker;

impl SrfSchemaMarker for JsonTreeMarker {
    type Schema = JsonTreeSchema;

    #[inline(always)]
    fn schema() -> &'static Self::Schema {
        &JSON_TREE_DEFAULT
    }
}

impl<'a, T, X> SrfBuildMarker<'a, T, X> for JsonTreeMarker
    where
        T: IntoSrfFunction<'a, X> {
    #[inline(always)]
    fn build_srf(args: T) -> ExtSrfFunction<'a> {
        let parsed = args.into_srf_function();
        ExtSrfFunction::JsonTree(parsed.base, parsed.path)
    }
}
