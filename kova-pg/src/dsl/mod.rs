mod assignment;
mod column;
mod conflict;
mod function;
mod jsonb_chain;
mod jsonb_text;
mod locking;
mod operator;
mod column_virtual;

pub mod func {
    pub use crate::dsl::function::{
        ExtFunction, ExtTableFunction,
        All, Any, ArrayAgg, JsonbAgg, JsonbObject, RowToJson,
        CumeDist, DenseRank, Lag, Lead, Ntile, PercentRank, Rank, RowNumber,
        IntoSeriesOp, GenerateSeries, JsonbArrayElements, JsonbEach, JsonbEachText, Unnest,       
    };
    
    pub(crate) use crate::dsl::function::{
        IntoTableFunction,
    };
}

pub use {
    assignment::ExtAssignment,
    column::ExtColumn,
    column_virtual::ExtVirtualColumn,
    conflict::Constraint,
    function::ExtFunction,
    locking::{LockMode, WaitPolicy},
};

pub(crate) use {
    conflict::{ConflictTarget, OnConflict},
    locking::{LockItem, Locking},
    operator::ExtOperator,
};

#[rustfmt::skip]
pub mod private {
    use kova_core::dsl::{Column, VirtualColumn};
    use kova_core::types::{ColumnMeta, IdentifierMeta};
    use crate::types::Postgres;
    use crate::dsl::{ExtColumn, ExtVirtualColumn};

    pub const fn create_column<T>(source: &'static str, name: &'static str, is_auto: bool) 
        -> Column<T, Postgres> {
        let meta = ColumnMeta::new(source, name, is_auto);
        Column::new(meta, ExtColumn::new(meta, create_virtual_column(source, name)))
    }

    pub const fn create_virtual_column<T, S>(source: &'static str, name: &'static str) 
        -> VirtualColumn<Postgres, T, S> {
        let meta = IdentifierMeta::new(source, name);
        VirtualColumn::new(meta, ExtVirtualColumn::new(meta))
    }
}
