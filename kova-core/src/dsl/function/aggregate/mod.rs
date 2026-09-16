mod avg;
mod count;
mod count_all;
mod max;
mod min;
mod sum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    All,
    Distinct,
}

pub use {avg::Avg, count::Count, count_all::CountAll, max::Max, min::Min, sum::Sum};
