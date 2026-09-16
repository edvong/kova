mod array_agg;
mod jsonb_agg;
mod jsonb_object;
mod jsonb_to_recordset;
mod row_to_json;

use crate::dsl::function::array_agg::ArrayAggState;
use crate::types::Postgres;
use kova_core::dsl::{Operand, Projection};

pub use crate::dsl::function::array_agg::ArrayAgg;
pub use crate::dsl::function::jsonb_agg::JsonbAgg;
pub use crate::dsl::function::jsonb_object::JsonbObject;
use crate::dsl::operator::JsonOp;

pub enum ExtFunction<'a> {
    JsonbObject(Vec<Projection<'a, Postgres, ()>>),
    JsonbAgg(Box<Projection<'a, Postgres, ()>>),
    ArrayAgg(ArrayAggState<'a, ()>),

    JsonChain {
        base: Box<Operand<'a, Postgres>>,
        ops: Vec<JsonOp<'a>>,
    },
}
