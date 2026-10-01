use crate::types::meta::SyncMeta;
use crate::types::{ColumnMeta, IdentifierMeta};

pub trait Extension<'a>: Sized {
    type Column<T>: SyncMeta<ColumnMeta> + Copy + Clone;

    type VirtualColumn<T>: SyncMeta<IdentifierMeta> + Copy + Clone;

    type Operator<'o> where Self: 'o;

    // Expression Extension
    type Function<'t> where Self: 't;

    type TableFunction<'t> where Self: 't;

    type OrderOperand;

    type Assignment<'t> where Self: 't;

    // Clause Extension
    type GroupBy;
    type Locking: Default;
    type OnConflict<'c>: Default;
}
