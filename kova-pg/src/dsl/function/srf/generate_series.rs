#![allow(non_snake_case, non_upper_case_globals)]

use crate::dsl::column::Column;
use crate::dsl::function::srf::ExtSrfFunction;
use crate::dsl::function::srf::srf_node::{SrfBuildMarker, SrfNode, SrfSchemaMarker};
use crate::types::Pg;
use kova_core::dsl::{Dynamic, IntoOperand};
use kova_core::types::IdentifierMeta;
use std::marker::PhantomData;

pub type GenerateSeries<L = (), R = (), S = (), K = ()> =
    SrfNode<SeriesArgs<L, R, S, K>, GenerateSeriesMarker>;

pub const fn GenerateSeries<L, R, KL, KR>(
    start: L, stop: R,
) -> GenerateSeries<L, R, L, (KL, KR, KL)> {
    SrfNode::new(SeriesArgs { start, stop, step: None, _marker: PhantomData })
}

pub struct SeriesArgs<L, R, S, K> {
    pub start: L,
    pub stop: R,
    pub step: Option<S>,
    pub _marker: PhantomData<K>,
}

impl<L: Copy, R: Copy, S: Copy, K> Copy for SeriesArgs<L, R, S, K> {}
impl<L: Clone, R: Clone, S: Clone, K> Clone for SeriesArgs<L, R, S, K> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            start: self.start.clone(),
            stop: self.stop.clone(),
            step: self.step.clone(),
            _marker: PhantomData,
        }
    }
}

impl<L, R, S, KR, KL, KS> GenerateSeries<L, R, S, (KL, KR, KS)> {
    #[inline(always)]
    pub fn step<S2, KS2>(self, step: S2) -> GenerateSeries<L, R, S2, (KL, KR, KS2)> {
        SrfNode::new(SeriesArgs {
            start: self.args.start,
            stop: self.args.stop,
            step: Some(step),
            _marker: PhantomData,
        })
    }
}

pub struct GenerateSeriesSchema {
    pub generate_series: Column<Dynamic, IdentifierMeta, Dynamic>,
}

pub const GENERATE_SERIES_DEFAULT: GenerateSeriesSchema =
    GenerateSeriesSchema { generate_series: Column::dynamic("generate_series", "generate_series") };

impl GenerateSeries<(), (), (), ()> {
    pub const generate_series: Column<Dynamic, IdentifierMeta, Dynamic> =
        GENERATE_SERIES_DEFAULT.generate_series;
}

pub struct GenerateSeriesMarker;

impl SrfSchemaMarker for GenerateSeriesMarker {
    type Schema = GenerateSeriesSchema;
    #[inline(always)]
    fn schema() -> &'static Self::Schema {
        &GENERATE_SERIES_DEFAULT
    }
}

impl<'a, L, R, S, KL, KR, KS, X> SrfBuildMarker<'a, SeriesArgs<L, R, S, (KL, KR, KS)>, X>
    for GenerateSeriesMarker
where
    L: IntoOperand<'a, Dynamic, Pg, KL, X>,
    R: IntoOperand<'a, Dynamic, Pg, KR, X>,
    S: IntoOperand<'a, Dynamic, Pg, KS, X>,
{
    #[inline(always)]
    fn build_srf(args: SeriesArgs<L, R, S, (KL, KR, KS)>) -> ExtSrfFunction<'a> {
        ExtSrfFunction::GenerateSeries {
            start: args.start.into_operand(),
            stop: args.stop.into_operand(),
            step: args.step.map(|s| s.into_operand()),
        }
    }
}
