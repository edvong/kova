use crate::dsl::can::CanGroup;
use crate::dsl::operand::MathOperator;
use crate::dsl::{AstNode, CanAlias, CanCompare, CanMath, CanOrder, Operand, TypedNode};
use crate::types::Database;
use std::marker::PhantomData;

// User::age.add(5).mul(2).gt(100) -> ((age + 5) * 2) > 100
pub struct Math<'a, DB: Database, Output, X> {
    op: Operand<'a, DB>,
    _marker: PhantomData<(Output, X)>,
}

impl<'a, DB: Database, Output, X> Math<'a, DB, Output, X> {
    #[inline(always)]
    pub fn new(op: MathOperator, left: Operand<'a, DB>, right: Operand<'a, DB>) -> Self {
        Self { op: Operand::from_math(op, left, right), _marker: PhantomData }
    }
}

impl<'a, DB: Database, Output, X> TypedNode for Math<'a, DB, Output, X> {
    type Output = Output;
}

impl<'a, DB: Database, Output, X> AstNode<'a, DB, X> for Math<'a, DB, Output, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, DB> {
        self.op
    }
}

impl<'a, DB: Database, Output, X> CanMath for Math<'a, DB, Output, X> {}
impl<'a, DB: Database, Output, X> CanCompare for Math<'a, DB, Output, X> {}
impl<'a, DB: Database, Output, X> CanAlias for Math<'a, DB, Output, X> {}
impl<'a, DB: Database, Output, X> CanOrder for Math<'a, DB, Output, X> {}
impl<'a, DB: Database, Output, X> CanGroup for Math<'a, DB, Output, X> {}
