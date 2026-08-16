use crate::dsl::operator::Operator;
use crate::types::Database;
use std::ops::{BitAnd, BitOr};

pub enum Condition<'a, DB: Database> {
    Op(Operator<'a, DB>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
}

// Gracefully supports `and` and `or` operators.
// e.g., `cond1 & cond2` AND `cond1 | cond2`
impl<'a, DB: Database> BitAnd for Condition<'a, DB> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Condition::And(Box::new(self), Box::new(rhs))
    }
}

impl<'a, DB: Database> BitOr for Condition<'a, DB> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Condition::Or(Box::new(self), Box::new(rhs))
    }
}
