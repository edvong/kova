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

mod can;
mod math;
mod relation;
pub mod statement;
mod table;

pub mod func {
    pub use crate::dsl::function::{
        Abs, AbsMarker, Aggregate, AggregateBuild, AggregateMarker, Avg, AvgMarker, Count,
        CountMarker, FrameBound, FrameType, Function, Length, LengthMarker, Lower, LowerMarker,
        Max, MaxMarker, Min, MinMarker, Quantifier, Scalar, ScalarBuild, ScalarMarker, SetDistinct,
        SetFilter, SetOrderBy, SetWindow, StandardFunction, Sum, SumMarker, Upper, UpperMarker,
        Wildcard, Window,
    };
}

pub use {
    arg::{IntoArg, ArgDirect, ArgAst, ArgEncode},
    assignment::{Assignment, StandardAssignment},
    can::{
        CanAlias, CanAssignment, CanCompare, CanDistinct, CanFilter, CanGroup, CanMath, CanOrder,
        CanOrderBy, CanString, CanWindow,
    },
    clause::*,
    column::Column,
    comparator::Comparator,
    component::Component,
    condition::Condition,
    identifier::{Identifier, IntoIdentifier},
    math::Math,
    operand::{
        AstNode, Close, Compatible, Dynamic, IntoArrayOutput, IntoOperand, JsonType, JsonbType,
        MathOperator, Open, Operand, Static, TypedNode,
    },
    operator::{Operator, StandardOperator},
    relation::IntoRelationList,
    table::Table,
};

pub trait EraseX {
    type Erased;

    fn erase_x(self) -> Self::Erased;
}
