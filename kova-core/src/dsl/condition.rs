use crate::dsl::component::WithComponent;
use crate::dsl::{
    Direction, IntoOperand, Operand, Operator, OrderOperand, Ordering, StandardOperand,
    StandardOperator,
};
use crate::types::Database;
use std::ops::{BitAnd, BitOr};

/// Represents a recursive logical condition in the SQL Abstract Syntax Tree (AST).
/// It can be a base operator (like equality) or a composite tree constructed via AND/OR logic.
pub enum Condition<'a, DB: Database> {
    Op(Operator<'a, DB>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
}

/// Enables ergonomic composition of conditions using the bitwise `&` operator,
/// mapping directly to the SQL `AND` logical operator.
// Gracefully supports `and` and `or` operators.
// e.g., `cond1 & cond2` AND `cond1 | cond2`
impl<'a, DB: Database> BitAnd for Condition<'a, DB> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Condition::And(Box::new(self), Box::new(rhs))
    }
}

/// Enables ergonomic composition of conditions using the bitwise `|` operator,
/// mapping directly to the SQL `OR` logical operator.
impl<'a, DB: Database> BitOr for Condition<'a, DB> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Condition::Or(Box::new(self), Box::new(rhs))
    }
}

impl<'a, DB: Database> Condition<'a, DB> {
    /// Transforms the boolean condition tree into an ascending `ORDER BY` expression.
    /// Takes ownership of the condition tree to avoid allocations.
    pub fn asc(self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Condition(self)),
            direction: Some(Direction::Asc),
            nulls: None,
        }
    }

    /// Transforms the boolean condition tree into a descending `ORDER BY` expression.
    /// Takes ownership of the condition tree to avoid allocations.
    pub fn desc(self) -> Ordering<'a, DB> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Condition(self)),
            direction: Some(Direction::Desc),
            nulls: None,
        }
    }
}

/// An internal helper trait that standardizes the construction of SQL conditional operators.
/// It abstracts away the boilerplate of wrapping operands into AST nodes.
pub(crate) trait ConditionBuilder<'a, DB: Database>: Sized {
    /// Requires the implementer to define the logic for converting itself into a generic `Operand`.
    fn build_operand(self) -> Operand<'a, DB>;

    /// Constructs a binary condition (e.g., `eq`, `ne`, `gt`, `lt`, `like`).
    ///
    /// **Core mechanism:** Accepts a function pointer (or closure) that maps two operands
    /// to a `StandardOperator`. This perfectly matches the signature of standard enum
    /// tuple variant constructors (e.g., `StandardOperator::Eq`).
    #[inline(always)]
    fn binary_op<V, M, F>(self, v: V, op_fn: F) -> Condition<'a, DB>
    where
        V: IntoOperand<'a, M, DB>,
        F: FnOnce(Operand<'a, DB>, Operand<'a, DB>) -> StandardOperator<'a, DB>,
    {
        Condition::Op(Operator::from_std(op_fn(
            self.build_operand(),
            v.into_operand(),
        )))
    }

    /// Constructs a unary condition (e.g., `is_null`, `is_not_null`).
    #[inline(always)]
    fn unary_op<F>(self, op_fn: F) -> Condition<'a, DB>
    where
        F: FnOnce(Operand<'a, DB>) -> StandardOperator<'a, DB>,
    {
        Condition::Op(Operator::from_std(op_fn(self.build_operand())))
    }

    /// Constructs a ternary condition (e.g., `between`, `not_between`).
    #[inline(always)]
    fn ternary_op<L, M1, U, M2, F>(self, lower: L, upper: U, op_fn: F) -> Condition<'a, DB>
    where
        L: IntoOperand<'a, M1, DB>,
        U: IntoOperand<'a, M2, DB>,
        F: FnOnce(Operand<'a, DB>, Operand<'a, DB>, Operand<'a, DB>) -> StandardOperator<'a, DB>,
    {
        Condition::Op(Operator::from_std(op_fn(
            self.build_operand(),
            lower.into_operand(),
            upper.into_operand(),
        )))
    }

    /// Constructs a list-based condition (e.g., `in_list`, `not_in`) by evaluating
    /// an iterator of values into a vector of operands.
    #[inline(always)]
    fn list_op<I, V, M, F>(self, values: I, op_fn: F) -> Condition<'a, DB>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, M, DB>,
        F: FnOnce(Operand<'a, DB>, Vec<Operand<'a, DB>>) -> StandardOperator<'a, DB>,
    {
        let vec = values.into_iter().map(|val| val.into_operand()).collect();
        Condition::Op(Operator::from_std(op_fn(self.build_operand(), vec)))
    }
}
