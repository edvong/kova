mod aggregate;
mod any_all;
pub(crate) mod json;
mod srf;
mod standard;
mod window;

pub use crate::dsl::function::{
    standard::*,
    any_all::{All, Any},
};

pub use crate::dsl::function::aggregate::{ArrayAgg, JsonbAgg};

pub use crate::dsl::function::json::{
    jsonb_build_object::JsonbBuildObject, row_to_json::RowToJson,
    jsonb_object::JsonbObject, jsonb_object::JsonbObjectSingle,
};

pub use crate::dsl::function::srf::{
    ExtSrfFunction,
    GenerateSeries, JsonbArrayElements, JsonbEach, JsonbEachText, Unnest,
};

pub use crate::dsl::function::window::{
    lag_lead::{Lag, Lead}, ntile::Ntile,
    parameterless::{CumeDist, DenseRank, PercentRank, Rank, RowNumber},
};

pub(crate) use crate::dsl::function::srf::IntoSrfFunction;

use crate::dsl::operator::JsonOp;
use crate::types::Postgres;
use kova_kernel::dsl::func::Function;
use kova_kernel::dsl::{IntoOperand, Operand, Projection, Static};

pub enum ExtFunction<'a> {
    JsonbAgg(Box<Operand<'a, Postgres>>),
    ArrayAgg(Box<Operand<'a, Postgres>>),
    
    JsonbBuildObject(Vec<Projection<'a, Postgres, ()>>),
    JsonbObject{
        keys: Box<Operand<'a, Postgres>>,
        values: Option<Box<Operand<'a, Postgres>>>,
    },
    RowToJson(Box<Operand<'a, Postgres>>),

    JsonChain {
        base: Box<Operand<'a, Postgres>>,
        op: Box<JsonOp<'a>>,
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
    Ntile(Box<Operand<'a, Postgres>>),
    Lag {
        expr: Box<Operand<'a, Postgres>>,
        offset: Option<Box<Operand<'a, Postgres>>>,
        default: Option<Box<Operand<'a, Postgres>>>,
    },
    Lead {
        expr: Box<Operand<'a, Postgres>>,
        offset: Option<Box<Operand<'a, Postgres>>>,
        default: Option<Box<Operand<'a, Postgres>>>,
    },
}

impl<'a> ExtFunction<'a> {
    /// Turn any valid iterator straight into a PostgreSQL ARRAY[...] AST node.
    #[inline(always)]
    pub(crate) fn from_iter<I, V, K, X>(iter: I) -> Operand<'a, Postgres>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        let ops = iter.into_iter().map(|v| v.into_operand()).collect();
        Operand::from_function(Function::from(ExtFunction::Array(ops)))
    }
}
