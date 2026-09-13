use crate::dsl::function::{Function, Quantifier};
use crate::types::Database;


pub trait BuildFunction<'a, DB: Database>: Sized {
    fn build_func(self) -> Function<'a, DB>;
}

// ==========================================
// Aggregate Function Builder
// ==========================================
pub(crate) trait AggregateBuilder<'a, DB: Database>: BuildFunction<'a, DB> {
    fn build_aggr(self, quantifier: Quantifier) -> Function<'a, DB>;

    #[inline(always)]
    fn build_distinct(self) -> Function<'a, DB> {
        self.build_aggr(Quantifier::Distinct)
    }
}
