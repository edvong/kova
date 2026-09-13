mod array_agg;
mod jsonb_agg;
mod jsonb_object;
mod jsonb_to_recordset;
mod row_to_json;

use crate::dsl::function::array_agg::ArrayAggState;
use kova_core::dsl::Projection;
use crate::types::Postgres;

pub use crate::dsl::function::jsonb_object::JsonbObject;
pub use crate::dsl::function::jsonb_agg::JsonbAgg;
pub use crate::dsl::function::array_agg::ArrayAgg;

pub enum Function<'a> {
    JsonbObject(Vec<Projection<'a, Postgres, ()>>),
    JsonbAgg(Box<Projection<'a, Postgres, ()>>),
    ArrayAgg(ArrayAggState<'a, ()>),
}
