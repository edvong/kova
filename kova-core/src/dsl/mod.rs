mod arg;
mod assignment;
mod column;
mod component;
mod condition;
mod function;
mod operator;
mod operand;

mod clause;
mod rows;

pub mod func {
    pub use crate::dsl::function::{
        Function, FunctionBuilder,
        Avg, Count, CountAll,Max, Min, Sum,
        Length, Lower,  Upper, Abs,
    };
}

pub use {
    arg::{IntoArg, IntoArgs, SingleArgMarker},
    assignment::{Assignment, StandardAssignment},
    column::Column,
    component::{Component, WithComponent},
    condition::{Condition, ConditionBuilder},
    operand::{Operand, IntoOperand, IsValue, IsAst},
    operator::{Operator, StandardOperator},
    rows::{InsertRows, IntoRows},
    clause::*,
};


pub trait EraseX {
    type Erased;

    fn erase_x(self) -> Self::Erased;
}