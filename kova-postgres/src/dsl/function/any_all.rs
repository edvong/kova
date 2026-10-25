use crate::dsl::func::ExtFunction;
use crate::query::InnerSelect;
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

impl<'a, M, Target, S, X> IntoOperand<'a, Target, Postgres, (Close, Close), X>
    for Any<InnerSelect<'a, M, S, X>>
    where
        M: Model<Postgres, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let sub_op = Operand::from_select(self.0.statement);
        Operand::from_function(Function::from(ExtFunction::Any(Box::new(sub_op))))
    }
}

impl<'a, M, Target, S, X> IntoOperand<'a, Target, Postgres, (Close, Close), X>
    for All<InnerSelect<'a, M, S, X>>
    where
        M: Model<Postgres, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
        let sub_op = Operand::from_select(self.0.statement);
        Operand::from_function(Function::from(ExtFunction::All(Box::new(sub_op))))
    }
}
