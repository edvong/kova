mod any_all;
mod array_agg;
mod jsonb_agg;
mod jsonb_object;
mod jsonb_to_recordset;
mod row_to_json;
mod window;
mod from;

pub use crate::dsl::function::{
    any_all::{All, Any},
    array_agg::ArrayAgg,
    jsonb_agg::JsonbAgg,
    jsonb_object::JsonbObject,
    row_to_json::RowToJson,
};

pub use crate::dsl::function::window::{
    lag::Lag, lead::Lead, ntile::Ntile,
    parameterless::{CumeDist, DenseRank, PercentRank, Rank, RowNumber},
};
pub use crate::dsl::function::from::{
    ExtTableFunction,
    IntoSeriesOp, GenerateSeries, JsonbArrayElements, JsonbEach, JsonbEachText, Unnest,
};

pub(crate) use crate::dsl::function::from::IntoTableFunction;

use crate::dsl::operator::JsonOp;
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{Condition, IntoOperand, Operand, Ordering, Projection};

pub enum ExtFunction<'a> {
    JsonbObject(Vec<Projection<'a, Postgres, ()>>),
    JsonbAgg(Box<Operand<'a, Postgres>>),
    ArrayAgg {
        operand: Box<Operand<'a, Postgres>>,
        distinct: bool,
        order_by: Vec<Ordering<'a, Postgres, ()>>,
        filter: Option<Condition<'a, Postgres, ()>>,
    },
    RowToJson(Box<Operand<'a, Postgres>>),

    JsonChain {
        base: Box<Operand<'a, Postgres>>,
        ops: Vec<JsonOp<'a>>,
    },

    // Tacking on ANY and ALL nodes. Keep in mind these can be backed by either an Array or a Subselect.
    Any(Box<Operand<'a, Postgres>>),
    All(Box<Operand<'a, Postgres>>),

    // Explicit array constructor: mapping to ARRAY[$1, $2, $3]
    Array(Vec<Operand<'a, Postgres>>),

    // Parameterless Window Functions
    RowNumber,
    Rank,
    DenseRank,
    PercentRank,
    CumeDist,

    // Parameterized Window Functions
    Ntile(i32),
    Lag(Box<Operand<'a, Postgres>>, Option<i32>, Option<Box<Operand<'a, Postgres>>>),
    Lead(Box<Operand<'a, Postgres>>, Option<i32>, Option<Box<Operand<'a, Postgres>>>),
}

impl<'a> ExtFunction<'a> {
    /// Turn any valid iterator straight into a PostgreSQL ARRAY[...] AST node.
    #[inline(always)]
    pub(crate) fn from_iter<I, V, Target, K, X>(iter: I) -> Operand<'a, Postgres>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, Postgres, K, X> {
        let ops = iter.into_iter().map(|v| v.into_operand()).collect();
        Operand::from_function(Function::from(ExtFunction::Array(ops)))
    }
}
