mod generate_series;
mod jsonb_array_elements;
mod jsonb_each;
mod jsonb_each_text;
mod jsonb_to_recordset;
mod srf_node;
mod unnest;

pub use crate::dsl::function::srf::{
    generate_series::GenerateSeries, jsonb_array_elements::JsonbArrayElements,
    jsonb_each::JsonbEach, jsonb_each_text::JsonbEachText, unnest::Unnest,
};

use crate::dsl::can::{CanPgArray, CanPgJsonb};
use crate::dsl::node::Node;
use crate::types::{Encode, Postgres};
use kova_core::dsl::{AstNode, Operand};

pub enum ExtSrfFunction<'a> {
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

/// JSONB SRF input contract (`JsonbEach`, `JsonbEachText`, `JsonbArrayElements`)
pub trait IntoSrfFunction<'a, X> {
    fn into_srf_function(self) -> Operand<'a, Postgres>;
}

impl<'a, T, X> IntoSrfFunction<'a, X> for Node<T>
    where
        T: CanPgJsonb<X> + AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn into_srf_function(self) -> Operand<'a, Postgres> {
        self.0.into_ast_operand()
    }
}

/// Array SRF input contract (`Unnest`)
pub trait IntoArraySrf<'a, X> {
    fn into_array_srf(self) -> Operand<'a, Postgres>;
}

impl<'a, T, X> IntoArraySrf<'a, X> for Node<T>
    where
        T: CanPgArray<X> + AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn into_array_srf(self) -> Operand<'a, Postgres> {
        self.0.into_ast_operand()
    }
}

impl<'a, T, X> IntoArraySrf<'a, X> for Vec<T>
    where
        Vec<T>: Encode<'a, X> {
    fn into_array_srf(self) -> Operand<'a, Postgres> {
        Operand::from_result(self.encode())
    }
}

impl<'a, 'b, T, X> IntoArraySrf<'a, X> for &'b Vec<T>
    where
        &'b Vec<T>: Encode<'a, X> {
    #[inline(always)]
    fn into_array_srf(self) -> Operand<'a, Postgres> {
        Operand::from_result(self.encode())
    }
}

impl<'a, 'b, T, X> IntoArraySrf<'a, X> for &'b [T]
    where
        &'b [T]: Encode<'a, X> {
    #[inline(always)]
    fn into_array_srf(self) -> Operand<'a, Postgres> {
        Operand::from_result(self.encode())
    }
}

impl<'a, T, const N: usize, X> IntoArraySrf<'a, X> for [T; N]
    where
        [T; N]: Encode<'a, X> {
    #[inline(always)]
    fn into_array_srf(self) -> Operand<'a, Postgres> {
        Operand::from_result(self.encode())
    }
}

impl<'a, 'b, T, const N: usize, X> IntoArraySrf<'a, X> for &'b [T; N]
    where
        &'b [T; N]: Encode<'a, X> {
    #[inline(always)]
    fn into_array_srf(self) -> Operand<'a, Postgres> {
        Operand::from_result(self.encode())
    }
}
