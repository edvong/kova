use crate::dsl::ExtFunction;
use crate::query::{Dispatch, Select};
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{Close, IntoOperand, Open, Operand};
use kova_core::types::Model;

pub struct Any<I>(pub I);
pub struct All<I>(pub I);

impl<'a, I, V, Target, X> IntoOperand<'a, Target, Postgres, (Close, Open), X> for Any<I>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, Postgres, Open, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let operands = self.0.into_iter().map(|val| val.into_operand()).collect();
        let array_op = Operand::from_function(Function::from(ExtFunction::Array(operands)));
        Operand::from_function(Function::from(ExtFunction::Any(Box::new(array_op))))
    }
}

impl<'a, I, V, Target, X> IntoOperand<'a, Target, Postgres, (Close, Open), X> for All<I>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, Target, Postgres, Open, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let operands = self.0.into_iter().map(|val| val.into_operand()).collect();
        let array_op = Operand::from_function(Function::from(ExtFunction::Array(operands)));
        Operand::from_function(Function::from(ExtFunction::All(Box::new(array_op))))
    }
}

impl<'a, M, Target, D: Dispatch, TAG, X> IntoOperand<'a, Target, Postgres, (Close, Close), X>
    for Any<Select<'a, M, D, TAG, X>>
    where
        M: Model<DB = Postgres> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let sub_op = Operand::from_select(self.0.statement);
        Operand::from_function(Function::from(ExtFunction::Any(Box::new(sub_op))))
    }
}

impl<'a, M, Target, D: Dispatch, TAG, X> IntoOperand<'a, Target, Postgres, (Close, Close), X>
    for All<Select<'a, M, D, TAG, X>>
    where
        M: Model<DB = Postgres> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let sub_op = Operand::from_select(self.0.statement);
        Operand::from_function(Function::from(ExtFunction::All(Box::new(sub_op))))
    }
}
