mod generate_series;
mod jsonb_array_elements;
mod jsonb_each;
mod jsonb_each_text;
mod unnest;

pub use crate::dsl::function::from::{
    unnest::Unnest,
    generate_series::{GenerateSeries, IntoSeriesOp},
    jsonb_each::JsonbEach,
    jsonb_each_text::JsonbEachText,
    jsonb_array_elements::JsonbArrayElements,
};

use crate::types::Postgres;
use kova_core::dsl::{Column, Operand};
use kova_core::types::Encode;

pub enum ExtTableFunction<'a> {
    Unnest(Operand<'a, Postgres>),
    JsonbArrayElements(Operand<'a, Postgres>),
    JsonbEach(Operand<'a, Postgres>),
    JsonbEachText(Operand<'a, Postgres>),
    GenerateSeries {
        start: Operand<'a, Postgres>,
        stop: Operand<'a, Postgres>,
        step: Option<Operand<'a, Postgres>>,
    },
}

pub trait IntoTableFunction<'a, X> {
    fn into_table_function(self) -> Operand<'a, Postgres>;
}

impl<'a, U, X> IntoTableFunction<'a, X> for Column<'a, U, Postgres> {
    #[inline(always)]
    fn into_table_function(self) -> Operand<'a, Postgres> {
        Operand::from_identifier(self.meta())
    }
}

impl<'a, U, X> IntoTableFunction<'a, X> for &Column<'a, U, Postgres> {
    #[inline(always)]
    fn into_table_function(self) -> Operand<'a, Postgres> {
        Operand::from_identifier(self.meta())
    }
}

impl<'a, T, X> IntoTableFunction<'a, X> for Vec<T>
    where
        Vec<T>: Encode<'a, Postgres, X, Vec<T>>, {
    fn into_table_function(self) -> Operand<'a, Postgres> {
        Operand::from_result(self.encode())
    }
}
