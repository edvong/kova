use kova_core::types::ColumnMeta;

pub enum Assignment {
    ExcludedColumn(ColumnMeta),
}