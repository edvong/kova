mod assignment;
mod can;
mod column;
mod function;
mod node;
mod operator;

pub use {
    can::CanSqliteJson,
    column::Column,
    node::Node,
};

pub(crate) use {
    assignment::ExtAssignment,
    // can::{CanSqliteString, CanSqliteFts},
    function::ExtFunction,
    operator::ExtOperator,
};


pub mod func {
    pub use crate::dsl::function::{
        Abs, Avg, Bm25, Count, GroupConcat, Highlight, Json, JsonEach, JsonExtract,
        JsonGroupArray, JsonObject, JsonTree, JsonbExtract, JsonbGroupArray, JsonbObject, Length,
        Lower, Math, Max, Min, Snippet, Sum, Upper,
    };

    // pub(crate) use crate::dsl::function::ExtFunction
    pub(crate) use crate::dsl::function::{IntoSrfFunction, JsonSrfArgs, ExtSrfFunction};
}
