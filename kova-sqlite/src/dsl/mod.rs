mod assignment;
mod column;
mod column_virtual;
mod conflict;
mod function;
mod operator;

pub use {column::ExtColumn, column_virtual::ExtVirtualColumn};

pub(crate) use {
    assignment::ExtAssignment,
    conflict::{ConflictTarget, OnConflict},
    function::ExtFunction,
    operator::ExtOperator,
};

pub mod func {
    pub use crate::dsl::function::GroupConcat;
    pub use crate::dsl::function::Json;
    pub use crate::dsl::function::JsonExtract;
    pub use crate::dsl::function::JsonGroupArray;
    pub use crate::dsl::function::JsonObject;
    pub use crate::dsl::function::JsonbExtract;
    pub use crate::dsl::function::JsonbGroupArray;
    pub use crate::dsl::function::JsonbObject;

    pub use crate::dsl::function::ExtTableFunction;

    pub use crate::dsl::function::Bm25;
    pub use crate::dsl::function::Highlight;
    pub use crate::dsl::function::Snippet;

    pub use crate::dsl::function::JsonEach;
    pub use crate::dsl::function::JsonTree;
}

#[rustfmt::skip]
pub mod private {
    use crate::dsl::{ExtColumn, ExtVirtualColumn};
    use crate::types::Sqlite;
    use kova_core::dsl::{Column, VirtualColumn};
    use kova_core::types::{ColumnMeta, IdentifierMeta};

    pub const fn create_column<T>(source: &'static str, name: &'static str, is_auto: bool) 
        -> Column<T, Sqlite> {
        let meta = ColumnMeta::new(source, name, is_auto);
        Column::new(meta, ExtColumn::new(meta, create_virtual_column(source, name)))
    }

    pub const fn create_virtual_column<T, S>(source: &'static str, name: &'static str) 
        -> VirtualColumn<Sqlite, T, S> {
        let meta = IdentifierMeta::new(source, name);
        VirtualColumn::new(meta, ExtVirtualColumn::new(meta))
    }
}
