
#[derive(Debug, Copy, Clone)]
pub struct ColumnMeta {
    pub table_name: &'static str,
    pub column_name: &'static str,
    pub is_autoincrement: bool,
    pub is_json: bool,
    pub is_ignore: bool,
}