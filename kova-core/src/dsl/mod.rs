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

mod relation;
pub mod statement;
mod virtual_column;

pub mod func {
    pub use crate::dsl::function::{
        Abs, Avg, Count, Star, FrameBound, FrameType, Function, IntoFunction, Length, Lower,
        Max, Min, Sum, Upper, Window,
    };
}

pub use {
    arg::{IntoArg, IntoArgs, SingleArgMarker},
    assignment::{Assignment, StandardAssignment},
    clause::*,
    column::Column,
    virtual_column::VirtualColumn,
    comparator::{AsComparator, Comparator, IntoComparator},
    component::{Component, WithComponent},
    condition::Condition,
    identifier::{Identifier, IntoIdentifier},
    operand::{IntoOperand, OpAst, OpValue, Operand},
    operator::{Operator, StandardOperator},
    relation::IntoRelationList,
    rows::{InsertRows, IntoInsertSource, IntoRows},
};

pub trait EraseX {
    type Erased;

    fn erase_x(self) -> Self::Erased;
}
