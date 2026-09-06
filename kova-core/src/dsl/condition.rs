use crate::dsl::component::WithComponent;
use crate::dsl::{
    Direction, EraseX, IntoOperand, Operand, Operator, OrderOperand, Ordering, StandardOperand,
    StandardOperator,
};
use crate::types::Database;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr};

pub struct Condition<'a, DB: Database, X> {
    node: ConditionNode<'a, DB>,
    _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> From<ConditionNode<'a, DB>> for Condition<'a, DB, X> {
    fn from(value: ConditionNode<'a, DB>) -> Self {
        Self {
            node: value,
            _marker: PhantomData,
        }
    }
}

impl<'a, DB: Database, X> EraseX for Condition<'a, DB, X> {
    type Erased = Condition<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Condition {
            node: self.node,
            _marker: PhantomData,
        }
    }
}

pub enum ConditionNode<'a, DB: Database> {
    Op(Operator<'a, DB>),
    And(Vec<Self>), // N-ary tree, contiguous memory
    Or(Vec<Self>),
}

impl<'a, DB: Database, X> BitAnd for Condition<'a, DB, X> {
    type Output = Self;

    #[inline(always)]
    fn bitand(mut self, rhs: Self) -> Self::Output {
        match &mut self.node {
            ConditionNode::And(nodes) => {
                nodes.push(rhs.node);
                self
            }
            _ => Condition {
                node: ConditionNode::And(vec![self.node, rhs.node]),
                _marker: PhantomData,
            },
        }
    }
}

impl<'a, DB: Database, X> BitOr for Condition<'a, DB, X> {
    type Output = Self;

    #[inline(always)]
    fn bitor(mut self, rhs: Self) -> Self::Output {
        match &mut self.node {
            ConditionNode::Or(nodes) => {
                nodes.push(rhs.node);
                self
            }
            _ => Condition {
                node: ConditionNode::Or(vec![self.node, rhs.node]),
                _marker: PhantomData,
            },
        }
    }
}

impl<'a, DB: Database, X> Condition<'a, DB, X> {
    /// Transforms the boolean condition tree into an ascending `ORDER BY` expression.
    /// Takes ownership of the condition tree to avoid allocations.
    pub fn asc(self) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Condition(self.erase_x())),
            direction: Some(Direction::Asc),
            nulls: None,
            _marker: PhantomData,
        }
    }

    /// Transforms the boolean condition tree into a descending `ORDER BY` expression.
    /// Takes ownership of the condition tree to avoid allocations.
    pub fn desc(self) -> Ordering<'a, DB, X> {
        Ordering {
            operand: OrderOperand::from_std(StandardOperand::Condition(self.erase_x())),
            direction: Some(Direction::Desc),
            nulls: None,
            _marker: PhantomData,
        }
    }
}

/// An internal helper trait that standardizes the construction of SQL conditional operators.
/// It abstracts away the boilerplate of wrapping operands into AST nodes.
pub trait ConditionBuilder<'a, DB: Database>: Sized {
    /// Requires the implementer to define the logic for converting itself into a generic `Operand`.
    fn build_operand(self) -> Operand<'a, DB>;

    /// Constructs a binary condition (e.g., `eq`, `ne`, `gt`, `lt`, `like`).
    ///
    /// **Core mechanism:** Accepts a function pointer (or closure) that maps two operands
    /// to a `StandardOperator`. This perfectly matches the signature of standard enum
    /// tuple variant constructors (e.g., `StandardOperator::Eq`).
    #[inline(always)]
    fn binary_op<V, Target, X, F>(self, v: V, op_fn: F) -> Condition<'a, DB, X>
    where
        V: IntoOperand<'a, Target, DB>,
        F: FnOnce(Operand<'a, DB>, Operand<'a, DB>) -> StandardOperator<'a, DB>,
    {
        ConditionNode::Op(Operator::from_std(op_fn(
            self.build_operand(),
            v.into_operand(),
        )))
        .into()
    }

    /// Constructs a unary condition (e.g., `is_null`, `is_not_null`).
    #[inline(always)]
    fn unary_op<F, X>(self, op_fn: F) -> Condition<'a, DB, X>
    where
        F: FnOnce(Operand<'a, DB>) -> StandardOperator<'a, DB>,
    {
        ConditionNode::Op(Operator::from_std(op_fn(self.build_operand()))).into()
    }

    /// Constructs a ternary condition (e.g., `between`, `not_between`).
    #[inline(always)]
    fn ternary_op<L, U, Target, X, F>(self, lower: L, upper: U, op_fn: F) -> Condition<'a, DB, X>
    where
        L: IntoOperand<'a, Target, DB>,
        U: IntoOperand<'a, Target, DB>,
        F: FnOnce(Operand<'a, DB>, Operand<'a, DB>, Operand<'a, DB>) -> StandardOperator<'a, DB>,
    {
        ConditionNode::Op(Operator::from_std(op_fn(
            self.build_operand(),
            lower.into_operand(),
            upper.into_operand(),
        )))
        .into()
    }

    /// Constructs a list-based condition (e.g., `in_list`, `not_in`) by evaluating
    /// an iterator of values into a vector of operands.
    #[inline(always)]
    fn list_op<I, V, Target, X, F>(self, values: I, op_fn: F) -> Condition<'a, DB, X>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, DB>,
        F: FnOnce(Operand<'a, DB>, Vec<Operand<'a, DB>>) -> StandardOperator<'a, DB>,
    {
        let vec = values.into_iter().map(|val| val.into_operand()).collect();
        ConditionNode::Op(Operator::from_std(op_fn(self.build_operand(), vec))).into()
    }
}
