use crate::types::Sqlite;
use kova_kernel::dsl::Operand;

pub enum ExtOperator<'a> {
    Match(Operand<'a, Sqlite>, Operand<'a, Sqlite>),
    Regexp(Operand<'a, Sqlite>, Operand<'a, Sqlite>),
}
