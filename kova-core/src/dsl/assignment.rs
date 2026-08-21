use crate::dsl::expression::Expression;
use crate::types::{ColumnMeta, Database};

pub enum Assignment<'a, DB: Database> {
    /// Standard value assignment.
    /// Generates: `col = ?` or `col = other_col`
    Value(ColumnMeta, Expression<'a, DB>),

    // Extensions
}


// pub type Assignment<'a, DB> = Component<
//     StandardAssignment<'a, DB>,
//     <DB as Database>::Extension<'a>::Assignment<'a>
// >;
