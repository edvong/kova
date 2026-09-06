mod array_agg;

pub use crate::dsl::function::array_agg::ArrayAgg;
use crate::dsl::function::array_agg::ArrayAggState;

/// Container for PostgreSQL-specific dialect functions.
///
/// This enum acts as an extension point, wrapping specialized AST nodes
/// (like `ArrayAgg`) so they can be seamlessly injected into the core SQL AST.
pub enum Function<'a> {
    ArrayAgg(ArrayAggState<'a, ()>),
}

