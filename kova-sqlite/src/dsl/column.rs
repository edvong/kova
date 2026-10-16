use crate::dsl::node::{CanSqliteFts, CanSqliteJson, CanSqliteString, Node};
use crate::types::Sqlite;
use kova_core::dsl::{
    Dynamic, IntoArg, IntoIdentifier, IntoJoinConstraint, JoinConstraint, Static,
};
use kova_core::types::{ColumnMeta, IdentifierMeta};

pub type Column<T = Dynamic, M = ColumnMeta, Mode = Static<T>> =
    Node<kova_core::dsl::Column<T, M, Mode>>;

impl<T> Column<T, ColumnMeta, Static<T>> {
    pub const fn new(source: &'static str, name: &'static str, is_auto: bool) -> Self {
        Node(kova_core::dsl::Column::new(source, name, is_auto))
    }
}

impl<T> Column<T, IdentifierMeta, Static<T>> {
    pub const fn identifier(source: &'static str, name: &'static str) -> Self {
        Node(kova_core::dsl::Column::identifier(source, name))
    }
}

impl Column<Dynamic, IdentifierMeta, Dynamic> {
    pub const fn dynamic(source: &'static str, name: &'static str) -> Self {
        Node(kova_core::dsl::Column::dynamic(source, name))
    }
}

impl<T, M, Mode> CanSqliteJson for kova_core::dsl::Column<T, M, Mode> {}
impl<T, M, Mode> CanSqliteString for kova_core::dsl::Column<T, M, Mode> {}

impl<T, M, Mode> CanSqliteFts for kova_core::dsl::Column<T, M, Mode>
    where
        M: IntoIdentifier<T> + Copy {
    #[inline(always)]
    fn fts_table(&self) -> &'static str {
        self.source()
    }
}

// USING (col) Single Column Join Constraint
impl<'a, T, M, Mode, X> IntoJoinConstraint<'a, Sqlite, X> for Column<T, M, Mode>
    where
        M: IntoIdentifier<T> {
    #[inline(always)]
    fn into_constraint(self) -> JoinConstraint<'a, Sqlite> {
        JoinConstraint::Using(vec![self.0.into_identifier().meta])
    }
}

// USING ((col1, col2)) Tuple Join Constraint
impl<T, M, Mode> IntoArg<IdentifierMeta> for Column<T, M, Mode>
    where
        M: IntoIdentifier<T> {
    #[inline(always)]
    fn into_arg(self) -> IdentifierMeta {
        self.0.into_identifier().meta
    }
}
