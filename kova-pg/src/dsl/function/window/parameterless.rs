use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::WithComponent;
use kova_core::dsl::func::{Function, Window};

pub struct RowNumber;
pub struct Rank;
pub struct DenseRank;
pub struct PercentRank;
pub struct CumeDist;

impl RowNumber {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from_ext(ExtFunction::RowNumber))
    }
}

impl Rank {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from_ext(ExtFunction::Rank))
    }
}

impl DenseRank {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from_ext(ExtFunction::DenseRank))
    }
}

impl PercentRank {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from_ext(ExtFunction::PercentRank))
    }
}

impl CumeDist {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from_ext(ExtFunction::CumeDist))
    }
}
