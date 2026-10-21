use crate::types::Pg;
use kova_core::dsl::func::{Aggregate, Scalar};
use kova_core::dsl::{CanString, Column, Dynamic, JsonType, JsonbType, Static, TypedNode};
use std::collections::{BTreeMap, HashMap};

#[diagnostic::on_unimplemented(
    message = "PostgreSQL textual capability (`CanPgString<{X}>`) is not satisfied for `{Self}`",
    label = "expected a textual column or string expression",
    note = "if `{Self}` is a custom textual type, implement `CanString<{X}>` for it."
)]
pub trait CanPgString<X> {}

#[diagnostic::on_unimplemented(
    message = "PostgreSQL array capability (`CanPgArray<{X}>`) is not satisfied for `{Self}`",
    label = "expected an array column or `ArrayAgg` expression",
    note = "if `{Self}` is a custom array type, implement `CanPgArray<{X}>` for it."
)]
pub trait CanPgArray<X> {}

#[diagnostic::on_unimplemented(
    message = "PostgreSQL JSON/JSONB capability (`CanPgJsonb<{X}>`) is not satisfied for `{Self}`",
    label = "expected a JSON/JSONB column or JSON expression",
    note = "if `{Self}` is a custom JSON type (e.g., `serde_json::Value`), implement `CanPgJsonb<{X}>` for it."
)]
pub trait CanPgJsonb<X> {}

// ==========================================
// 1. Output Mode & Type Capability Implementations
// ==========================================

// Every type/node with `CanString<X>` automatically possesses `CanPgString<X>`
impl<T: ?Sized + CanString<X>, X> CanPgString<X> for T {}

impl<X> CanPgArray<X> for Dynamic {}
impl<X> CanPgJsonb<X> for Dynamic {}

impl<T: ?Sized + CanPgArray<X>, X> CanPgArray<X> for Static<T> {}
impl<'a, T: ?Sized + CanPgArray<X>, X> CanPgArray<X> for &'a T {}
impl<T: ?Sized + CanPgArray<X>, X> CanPgArray<X> for Box<T> {}
impl<T: CanPgArray<X>, X> CanPgArray<X> for Option<T> {}

impl<T: ?Sized + CanPgJsonb<X>, X> CanPgJsonb<X> for Static<T> {}
impl<'a, T: ?Sized + CanPgJsonb<X>, X> CanPgJsonb<X> for &'a T {}
impl<T: ?Sized + CanPgJsonb<X>, X> CanPgJsonb<X> for Box<T> {}
impl<T: CanPgJsonb<X>, X> CanPgJsonb<X> for Option<T> {}

impl<T, X> CanPgArray<X> for Vec<T> {}
impl<T, X> CanPgArray<X> for [T] {}
impl<T, const N: usize, X> CanPgArray<X> for [T; N] {}

impl<X> CanPgJsonb<X> for JsonType {}
impl<X> CanPgJsonb<X> for JsonbType {}
impl<K, V, S, X> CanPgJsonb<X> for HashMap<K, V, S> {}
impl<K, V, X> CanPgJsonb<X> for BTreeMap<K, V> {}

// ==========================================
// 2. Core AST Node Capability Derivations
// ==========================================

impl<T, M, Mode: CanPgArray<X>, X> CanPgArray<X> for Column<T, M, Mode> {}
impl<T, M, Mode: CanPgJsonb<X>, X> CanPgJsonb<X> for Column<T, M, Mode> {}

impl<M, E, X> CanPgArray<X> for Scalar<M, E>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanPgArray<X> {
}

impl<M, E, X> CanPgJsonb<X> for Scalar<M, E>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanPgJsonb<X> {
}

impl<'a, M, E, X> CanPgArray<X> for Aggregate<'a, Pg, M, E, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanPgArray<X> {
}

impl<'a, M, E, X> CanPgJsonb<X> for Aggregate<'a, Pg, M, E, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanPgJsonb<X> {
}
