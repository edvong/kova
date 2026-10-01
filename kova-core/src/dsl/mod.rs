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
mod column_virtual;

pub mod func {
    pub use crate::dsl::function::{
        Abs, Avg, Count, Star, Length, Lower, Max, Min, Sum, Upper,
        FrameBound, FrameType, Function, Window, IntoFunction,
    };
}

pub use {
    arg::{IntoArg, IntoArgs, SingleArgMarker},
    assignment::{Assignment, StandardAssignment},
    clause::*,
    column::Column,
    column_virtual::{VirtualColumn, Strict, Dynamic},
    comparator::{Comparator, IntoComparator},
    component::{Component, WithComponent},
    condition::Condition,
    identifier::{Identifier, IntoIdentifier},
    operand::{IntoOperand, OpNode, OpAst, OpValue, Operand},
    operator::{Operator, StandardOperator},
    relation::IntoRelationList,
    rows::{InsertRows, IntoInsertSource, IntoRows},
};

pub trait EraseX {
    type Erased;

    fn erase_x(self) -> Self::Erased;
}
