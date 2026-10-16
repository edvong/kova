use crate::dsl::node::{CanSqliteJson, CanSqliteString};
use crate::types::Sqlite;
use kova_core::dsl::func::{Aggregate, LowerMarker, Scalar, UpperMarker};

impl<'a, M: CanSqliteJson, E, X> CanSqliteJson for Aggregate<'a, Sqlite, M, E, X> {}
impl<'a, M: CanSqliteString, E, X> CanSqliteString for Aggregate<'a, Sqlite, M, E, X> {}

impl<M: CanSqliteJson, E> CanSqliteJson for Scalar<M, E> {}
impl<M: CanSqliteString, E> CanSqliteString for Scalar<M, E> {}

impl CanSqliteString for LowerMarker {}
impl CanSqliteString for UpperMarker {}

