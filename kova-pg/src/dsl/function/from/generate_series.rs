#![allow(non_upper_case_globals)]

use crate::dsl::function::from::ExtTableFunction;
use crate::types::Postgres;
use kova_core::dsl::{Column, IntoArg, Operand, SingleArgMarker, Table, VirtualColumn};
use kova_core::types::Encode;

pub struct GenerateSeries<L = (), R = ()>(pub L, pub R);

pub struct GenerateSeriesStep<Base, S> {
    base: Base,
    step: S,
}

impl GenerateSeries<(), ()> {
    pub const generate_series: VirtualColumn =
        VirtualColumn::new("generate_series", "generate_series");
}

impl<L, R> SingleArgMarker for GenerateSeries<L, R> {}
impl<Base, S> SingleArgMarker for GenerateSeriesStep<Base, S> {}

impl<L, R> GenerateSeries<L, R> {
    #[inline(always)]
    pub fn step<S>(self, step: S) -> GenerateSeriesStep<Self, S> {
        GenerateSeriesStep { base: self, step }
    }
}

pub trait IntoSeriesOp<'a, X> {
    fn into_series_op(self) -> Operand<'a, Postgres>;
}

impl<'a, T, X> IntoSeriesOp<'a, X> for Column<'a, T, Postgres> {
    #[inline(always)]
    fn into_series_op(self) -> Operand<'a, Postgres> {
        self.into_arg()
    }
}

impl<'a, 'b, X> IntoSeriesOp<'a, X> for &'b str
    where
        &'b str: Encode<'a, Postgres, X, String> {
    #[inline(always)]
    fn into_series_op(self) -> Operand<'a, Postgres> {
        Operand::from_result(self.encode())
    }
}

macro_rules! impl_native_series_op {
    ($($ty:ty),+) => {
        $(
            impl<'a, X> IntoSeriesOp<'a, X> for $ty
                where
                    $ty: Encode<'a, Postgres, X, $ty> {
                #[inline(always)]
                fn into_series_op(self) -> Operand<'a, Postgres> {
                    Operand::from_result(self.encode())
                }
            }
        )+
    };
}
impl_native_series_op!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, String);

impl<'a, L, R, X> IntoArg<Table<'a, Postgres, X>> for GenerateSeries<L, R>
    where
        L: IntoSeriesOp<'a, X>,
        R: IntoSeriesOp<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> Table<'a, Postgres, X> {
        let func = ExtTableFunction::GenerateSeries {
            start: self.0.into_series_op(),
            stop: self.1.into_series_op(),
            step: None,
        };
        Table::from_function(func, None)
    }
}

impl<'a, L, R, S, X> IntoArg<Table<'a, Postgres, X>> for GenerateSeriesStep<GenerateSeries<L, R>, S>
    where
        L: IntoSeriesOp<'a, X>,
        R: IntoSeriesOp<'a, X>,
        S: IntoSeriesOp<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> Table<'a, Postgres, X> {
        let func = ExtTableFunction::GenerateSeries {
            start: self.base.0.into_series_op(),
            stop: self.base.1.into_series_op(),
            step: Some(self.step.into_series_op()),
        };
        Table::from_function(func, None)
    }
}
