use crate::dsl::{ArgAst, Direction, EraseX, IntoArg, IntoOrdering, Operator, Ordering};
use crate::types::Database;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, Not};

pub struct Condition<'a, DB: Database, X> {
    node: ConditionNode<'a, DB>,
    _marker: PhantomData<X>,
}

enum ConditionNode<'a, DB: Database> {
    Op(Box<Operator<'a, DB>>),
    And(Vec<Self>), // N-ary tree, contiguous memory
    Or(Vec<Self>),
    Not(Box<Self>),
}

impl<'a, DB: Database, X> Condition<'a, DB, X> {
    pub fn from(operator: Operator<'a, DB>) -> Self {
        Self { node: ConditionNode::Op(Box::new(operator)), _marker: PhantomData }
    }
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

impl<'a, DB: Database, X> Not for Condition<'a, DB, X> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Condition { node: ConditionNode::Not(Box::new(self.node)), _marker: PhantomData }
    }
}

impl<'a, DB: Database, N, X> IntoOrdering<'a, DB, X> for Condition<'a, DB, N> {
    fn into_ordering(self) -> Ordering<'a, DB, X> {
        Ordering::from_condition(self.erase_x())
    }
}

impl<'a, DB: Database, X> IntoArg<Ordering<'a, DB, X>, ArgAst> for Condition<'a, DB, X> {
    fn into_arg(self) -> Ordering<'a, DB, X> {
        self.into_ordering().direction(Direction::Asc)
    }
}

impl<'a, DB: Database, X> EraseX for Condition<'a, DB, X> {
    type Erased = Condition<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Condition { node: self.node, _marker: PhantomData }
    }
}
