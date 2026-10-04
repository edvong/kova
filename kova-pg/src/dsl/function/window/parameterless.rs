use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::{Function, Window};

pub struct RowNumber;
pub struct Rank;
pub struct DenseRank;
pub struct PercentRank;
pub struct CumeDist;

impl RowNumber {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from(ExtFunction::RowNumber))
    }
}

impl Rank {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from(ExtFunction::Rank))
    }
}

impl DenseRank {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from(ExtFunction::DenseRank))
    }
}

impl PercentRank {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from(ExtFunction::PercentRank))
    }
}

impl CumeDist {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from(ExtFunction::CumeDist))
    }
}
