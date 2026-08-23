use crate::types::{ColumnMeta, Database};
use std::marker::PhantomData;

// pub type Assignment<'a, DB> = Component<
//     StandardAssignment<'a, DB>,
//     <DB as Database>::Extension<'a>::Assignment<'a>
// >;

pub enum Assignment<'a, DB: Database> {
    /// Standard value assignment.
    /// Generates: `col = ?` or `col = other_col`
    Value(ColumnMeta, PhantomData<&'a DB>),
    // Value(ColumnMeta, Expression<'a, DB>),

    // Extensions
}
