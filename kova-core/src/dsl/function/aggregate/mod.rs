mod count_all;
mod sum;
mod max;
mod min;
mod avg;
mod count;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    All,
    Distinct,
}

pub use {
    count_all::CountAll,
    count::Count,
    sum::Sum,
    avg::Avg,
    max::Max,
    min::Min,
};
