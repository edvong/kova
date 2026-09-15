mod arg;
mod assignment;
mod column;
mod component;
mod condition;
mod function;
mod operand;
mod operator;

mod alias;
mod clause;
mod rows;
mod comparator;

pub mod func {
    pub use crate::dsl::function::{
        Abs, Avg, BuildFunction, Count, CountAll, Function, Length, Lower, Max, Min, Sum, Upper,
    };
}

pub use {
    alias::{Alias, Identifier, IntoAlias},
    arg::{IntoArg, IntoArgs, SingleArgMarker},
    assignment::{Assignment, StandardAssignment},
    clause::*,
    column::Column,
    component::{Component, WithComponent},
    condition::{Condition},
    operand::{IntoOperand, IsAst, IsValue, Operand},
    operator::{Operator, StandardOperator},
    comparator::{Comparator, IntoComparator, AsComparator},
    rows::{InsertRows, IntoRows},
};

pub trait EraseX {
    type Erased;

    fn erase_x(self) -> Self::Erased;
}
