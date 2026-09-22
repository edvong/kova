mod arg;
mod assignment;
mod clause;
mod column;
mod comparator;
mod component;
mod condition;
mod function;
mod identifier;
mod operand;
mod operator;
mod rows;

pub mod statement;
mod relation;

pub mod func {
    pub use crate::dsl::function::{
        Abs, Avg, Count, CountAll, FrameBound, FrameType, Function, IntoFunction, Length, Lower,
        Max, Min, Sum, Upper, Window,
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
    relation::IntoRelationList,
};

pub trait EraseX {
    type Erased;

    fn erase_x(self) -> Self::Erased;
}
