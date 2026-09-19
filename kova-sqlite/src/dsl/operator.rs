use crate::types::Sqlite;
use kova_core::dsl::Operand;

pub enum ExtOperator<'a> {
    Match(Operand<'a, Sqlite>, Operand<'a, Sqlite>),
    Regexp(Operand<'a, Sqlite>, Operand<'a, Sqlite>),
}
