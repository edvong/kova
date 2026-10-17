use crate::dsl::node::{CanPgArray, CanPgJsonb, CanPgString, Node};
use kova_core::dsl::{ArgAst, Dynamic, IntoArg, IntoIdentifier, Static};
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

impl<T, M, Mode> CanPgString for kova_core::dsl::Column<T, M, Mode> {}
impl<T, M, Mode> CanPgArray for kova_core::dsl::Column<T, M, Mode> {}
impl<T, M, Mode> CanPgJsonb for kova_core::dsl::Column<T, M, Mode> {}

impl<T, M, Mode> IntoArg<IdentifierMeta, ArgAst> for Column<T, M, Mode>
    where
        M: IntoIdentifier<T> {
    #[inline(always)]
    fn into_arg(self) -> IdentifierMeta {
        self.0.into_identifier().meta
    }
}
