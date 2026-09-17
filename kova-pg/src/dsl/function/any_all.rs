use crate::dsl::subselect::Chain;
use crate::dsl::{ExtFunction, Subselect};
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{IntoOperand, IsAst, IsValue, Operand, WithComponent};
use kova_core::types::Model;

pub struct Any<I>(pub I);
pub struct All<I>(pub I);

// Core thought: As long as the elements V in the collection can be converted into an Operand,
// the collection I itself is good to go as an argument for Any.
impl<'a, I, V, Target, X> IntoOperand<'a, Target, Postgres, IsAst, X> for Any<I>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, Postgres, IsValue, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let operands: Vec<Operand<'a, Postgres>> =
            self.0.into_iter().map(|val| val.into_operand()).collect();
        let array_op = Operand::from_function(Function::from_ext(ExtFunction::Array(operands)));
        Operand::from_function(Function::from_ext(ExtFunction::Any(Box::new(array_op))))
    }
}

// Implementation for All follows the exact same pattern.
impl<'a, I, V, Target, X> IntoOperand<'a, Target, Postgres, IsAst, X> for All<I>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, Postgres, IsValue, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let operands: Vec<Operand<'a, Postgres>> =
            self.0.into_iter().map(|val| val.into_operand()).collect();
        let array_op = Operand::from_function(Function::from_ext(ExtFunction::Array(operands)));
        Operand::from_function(Function::from_ext(ExtFunction::All(Box::new(array_op))))
    }
}

impl<'a, M, Target, X> IntoOperand<'a, Target, Postgres, IsAst, X>
    for Any<Subselect<'a, M, Chain<Target>>>
    where
        M: Model<DB = Postgres> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        // Just extract the underlying SelectStatement directly to build out the subquery Operand.
        let sub_op = Operand::from_subselect(self.0.statement);

        // Then simply wrap it straight into ANY(...)
        Operand::from_function(Function::from_ext(ExtFunction::Any(Box::new(sub_op))))
    }
}

// Same trick applies to All.
impl<'a, M, Target, X> IntoOperand<'a, Target, Postgres, IsAst, X>
    for All<Subselect<'a, M, Chain<Target>>>
    where
        M: Model<DB = Postgres> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let sub_op = Operand::from_subselect(self.0.statement);
        Operand::from_function(Function::from_ext(ExtFunction::All(Box::new(sub_op))))
    }
}
