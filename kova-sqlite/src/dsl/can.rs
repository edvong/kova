use crate::types::Sqlite;
use kova_kernel::dsl::func::{Aggregate, Scalar};
use kova_kernel::dsl::{
    CanString, Column, Dynamic, JsonType, JsonbType, Static, TypedNode,
};
use kova_kernel::types::ColumnMeta;
use std::collections::{BTreeMap, HashMap};

#[diagnostic::on_unimplemented(
    message = "SQLite JSON/JSONB capability (`CanSqliteJson<{X}>`) is not satisfied for `{Self}`",
    label = "expected a JSON, TEXT, or JSONB BLOB column/expression",
    note = "if `{Self}` is a custom JSON type (e.g., `serde_json::Value`), implement `CanSqliteJson<{X}>` for it."
)]
pub trait CanSqliteJson<X> {}

#[diagnostic::on_unimplemented(
    message = "SQLite textual capability (`CanSqliteString<{X}>`) is not satisfied for `{Self}`",
    label = "expected a textual column or string expression",
    note = "if `{Self}` is a custom textual type, implement `CanString<{X}>` for it."
)]
pub trait CanSqliteString<X> {}

#[diagnostic::on_unimplemented(
    message = "SQLite FTS5 capability (`CanSqliteFts<{X}>`) is not satisfied for `{Self}`",
    label = "expected a textual FTS5 table column"
)]
pub trait CanSqliteFts<X> {
    fn fts_table(&self) -> &'static str;
}

// ==========================================
// 1. Output Mode & Type Capability Implementations
// ==========================================

// Every type/node with `CanString<X>` automatically possesses `CanSqliteString<X>`
impl<T: ?Sized + CanString<X>, X> CanSqliteString<X> for T {}

impl<X> CanSqliteJson<X> for Dynamic {}

impl<T: ?Sized + CanSqliteJson<X>, X> CanSqliteJson<X> for Static<T> {}
impl<'a, T: ?Sized + CanSqliteJson<X>, X> CanSqliteJson<X> for &'a T {}
impl<T: ?Sized + CanSqliteJson<X>, X> CanSqliteJson<X> for Box<T> {}
impl<T: CanSqliteJson<X>, X> CanSqliteJson<X> for Option<T> {}

impl<X> CanSqliteJson<X> for JsonType {}
impl<X> CanSqliteJson<X> for JsonbType {}
impl<X> CanSqliteJson<X> for String {}
impl<X> CanSqliteJson<X> for str {}
impl<X> CanSqliteJson<X> for Vec<u8> {}
impl<X> CanSqliteJson<X> for [u8] {}
impl<K, V, S, X> CanSqliteJson<X> for HashMap<K, V, S> {}
impl<K, V, X> CanSqliteJson<X> for BTreeMap<K, V> {}

// ==========================================
// 2. Core AST Node Capability Derivations
// ==========================================

impl<T, M, Mode: CanSqliteJson<X>, X> CanSqliteJson<X> for Column<T, M, Mode> {}

impl<T, Mode: CanString<X>, X> CanSqliteFts<X> for Column<T, ColumnMeta, Mode> {
    #[inline(always)]
    fn fts_table(&self) -> &'static str {
        self.source()
    }
}

impl<M, E, X> CanSqliteJson<X> for Scalar<M, E>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanSqliteJson<X> {
}

impl<'a, M, E, X> CanSqliteJson<X> for Aggregate<'a, Sqlite, M, E, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanSqliteJson<X> {
}
