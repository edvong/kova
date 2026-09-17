mod any_all;
mod array_agg;
mod jsonb_agg;
mod jsonb_object;
mod jsonb_to_recordset;
mod row_to_json;

use crate::dsl::function::array_agg::ArrayAggState;
use crate::dsl::operator::JsonOp;
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{IntoOperand, Operand, Projection, WithComponent};

pub use crate::dsl::function::any_all::{All, Any};
pub use crate::dsl::function::array_agg::ArrayAgg;
pub use crate::dsl::function::jsonb_agg::JsonbAgg;
pub use crate::dsl::function::jsonb_object::JsonbObject;

pub enum ExtFunction<'a> {
    JsonbObject(Vec<Projection<'a, Postgres, ()>>),
    JsonbAgg(Box<Projection<'a, Postgres, ()>>),
    ArrayAgg(ArrayAggState<'a, ()>),

    JsonChain {
        base: Box<Operand<'a, Postgres>>,
        ops: Vec<JsonOp<'a>>,
    },

    // Tacking on ANY and ALL nodes. Keep in mind these can be backed by either an Array or a Subselect.
    Any(Box<Operand<'a, Postgres>>),
    All(Box<Operand<'a, Postgres>>),

    // Explicit array constructor: mapping to ARRAY[$1, $2, $3]
    Array(Vec<Operand<'a, Postgres>>),
}

impl<'a> ExtFunction<'a> {
    /// Turn any valid iterator straight into a PostgreSQL ARRAY[...] AST node.
    #[inline(always)]
    pub(crate) fn from_iter<I, V, Target, K, X>(iter: I) -> Operand<'a, Postgres>
        where
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Target, Postgres, K, X> {
        let ops = iter.into_iter().map(|v| v.into_operand()).collect();
        Operand::from_function(Function::from_ext(ExtFunction::Array(ops)))
    }
}
