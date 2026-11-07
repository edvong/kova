use crate::dsl::node::Node;
use kova_kernel::dsl::{ArgAst, Dynamic, IntoArg, IntoIdentifier, Static};
use kova_kernel::types::{ColumnMeta, IdentifierMeta};

pub type Column<T = Dynamic, M = ColumnMeta, Mode = Static<T>> =
    Node<kova_kernel::dsl::Column<T, M, Mode>>;

impl<T> Column<T, ColumnMeta, Static<T>> {
    pub const fn new(source: &'static str, name: &'static str, is_auto: bool) -> Self {
        Node(kova_kernel::dsl::Column::new(source, name, is_auto))
    }
}

impl<T> Column<T, IdentifierMeta, Static<T>> {
    pub const fn identifier(source: &'static str, name: &'static str) -> Self {
        Node(kova_kernel::dsl::Column::identifier(source, name))
    }
}

impl Column<Dynamic, IdentifierMeta, Dynamic> {
    pub const fn dynamic(source: &'static str, name: &'static str) -> Self {
        Node(kova_kernel::dsl::Column::dynamic(source, name))
    }
}

impl<T, M, Mode> IntoArg<IdentifierMeta, ArgAst> for Column<T, M, Mode>
    where
        M: IntoIdentifier<T> {
    #[inline(always)]
    fn into_arg(self) -> IdentifierMeta {
        self.0.into_identifier().meta
    }
}
