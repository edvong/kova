mod arg;
mod assignment;
mod column;
mod component;
mod condition;
mod function;
mod operand;
mod operator;

mod clause;
mod comparator;
mod identifier;
mod rows;

pub mod func {
    pub use crate::dsl::function::{
        Abs, Avg, IntoFunction, Count, CountAll, Function, Length, Lower, Max, Min, Sum, Upper,
    };
}

pub use {
    arg::{IntoArg, IntoArgs, SingleArgMarker},
    assignment::{Assignment, StandardAssignment},
    clause::*,
    column::Column,
    comparator::{AsComparator, Comparator, IntoComparator},
    component::{Component, WithComponent},
    condition::Condition,
    identifier::{Identifier, IntoIdentifier},
    operand::{IntoOperand, OpAst, OpValue, Operand},
    operator::{Operator, StandardOperator},
    rows::{InsertRows, IntoRows},
};

pub trait EraseX {
    type Erased;

    fn erase_x(self) -> Self::Erased;
}
