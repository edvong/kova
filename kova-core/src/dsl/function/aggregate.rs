use crate::dsl::function::{
    Function, Quantifier, SetDistinct, SetFilter, SetOrderBy, SetWindow, StandardFunction, Window,
};
use crate::dsl::{
    AstNode, CanAlias, CanCompare, CanDistinct, CanFilter, CanGroup, CanMath, CanOrder, CanOrderBy,
    CanString, CanWindow, Condition, Operand, Ordering, Static, TypedNode,
};
use crate::types::Database;
use std::marker::PhantomData;

pub type Avg<'a, DB, E, X> = Aggregate<'a, DB, AvgMarker, E, X>;
pub type Count<'a, DB, E, X> = Aggregate<'a, DB, CountMarker, E, X>;
pub type Max<'a, DB, E, X> = Aggregate<'a, DB, MaxMarker, E, X>;
pub type Min<'a, DB, E, X> = Aggregate<'a, DB, MinMarker, E, X>;
pub type Sum<'a, DB, E, X> = Aggregate<'a, DB, SumMarker, E, X>;

pub struct Aggregate<'a, DB: Database, M, E, X> {
    pub(crate) expr: E,
    pub(crate) quantifier: Option<Quantifier>,
    pub(crate) order_by: Vec<Ordering<'a, DB, ()>>,
    pub(crate) filter: Option<Condition<'a, DB, ()>>,
    pub(crate) window: Option<Window<'a, DB, ()>>,
    _marker: PhantomData<(M, X)>,
}

impl<'a, DB: Database, M, E, X> Aggregate<'a, DB, M, E, X> {
    #[inline(always)]
    pub const fn new(expr: E) -> Self {
        Self {
            expr,
            quantifier: None,
            order_by: Vec::new(),
            filter: None,
            window: None,
            _marker: PhantomData,
        }
    }
}

pub trait AggregateMarker<E> {
    type Output;
}

pub trait AggregateBuild<'a, DB: Database, E, X>: AggregateMarker<E> {
    fn build_function(expr: E) -> Function<'a, DB>;
}

impl<'a, DB: Database, M, E, X> TypedNode for Aggregate<'a, DB, M, E, X>
    where
        M: AggregateMarker<E> {
    type Output = M::Output;
}

impl<'a, DB: Database, M, E, X> AstNode<'a, DB, X> for Aggregate<'a, DB, M, E, X>
    where
        M: AggregateBuild<'a, DB, E, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, DB> {
        let mut f = M::build_function(self.expr);
        if let Some(q) = self.quantifier {
            f = f.quantifier(q);
        }
        if !self.order_by.is_empty() {
            f = f.order_by(self.order_by);
        }
        if let Some(filter) = self.filter {
            f = f.filter(filter);
        }
        if let Some(window) = self.window {
            f = f.window(window);
        }
        Operand::from_function(f)
    }
}

impl<'a, DB: Database, M, E, X> SetDistinct for Aggregate<'a, DB, M, E, X> {
    #[inline(always)]
    fn set_distinct(mut self, quantifier: Quantifier) -> Self {
        self.quantifier = Some(quantifier);
        self
    }
}

impl<'a, DB: Database, M, E, X> SetOrderBy<'a, DB, X> for Aggregate<'a, DB, M, E, X> {
    #[inline(always)]
    fn set_order_by(mut self, ordering: Vec<Ordering<'a, DB, ()>>) -> Self {
        self.order_by = ordering;
        self
    }
}

impl<'a, DB: Database, M, E, X> SetFilter<'a, DB, X> for Aggregate<'a, DB, M, E, X> {
    #[inline(always)]
    fn set_filter(mut self, cond: Condition<'a, DB, ()>) -> Self {
        self.filter = Some(cond);
        self
    }
}

impl<'a, DB: Database, M, E, X> SetWindow<'a, DB, X> for Aggregate<'a, DB, M, E, X> {
    #[inline(always)]
    fn set_window(mut self, window: Window<'a, DB, ()>) -> Self {
        self.window = Some(window);
        self
    }
}

// All aggregate functions inherently possess core capabilities.
impl<'a, DB: Database, M, E, X> CanAlias for Aggregate<'a, DB, M, E, X> {}
impl<'a, DB: Database, M, E, X> CanWindow for Aggregate<'a, DB, M, E, X> {}
impl<'a, DB: Database, M, E, X> CanFilter for Aggregate<'a, DB, M, E, X> {}
impl<'a, DB: Database, M, E, X> CanDistinct for Aggregate<'a, DB, M, E, X> {}
impl<'a, DB: Database, M, E, X> CanOrderBy for Aggregate<'a, DB, M, E, X> {}
impl<'a, DB: Database, M, E, X> CanCompare for Aggregate<'a, DB, M, E, X> {}
impl<'a, DB: Database, M, E, X> CanOrder for Aggregate<'a, DB, M, E, X> {}
impl<'a, DB: Database, M: CanGroup, E, X> CanGroup for Aggregate<'a, DB, M, E, X> {}

// Math & String capabilities derived from the resolved Output type
impl<'a, DB: Database, M, E, X> CanMath<X> for Aggregate<'a, DB, M, E, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanMath<X> {
}

impl<'a, DB: Database, M, E, X> CanString<X> for Aggregate<'a, DB, M, E, X>
    where
        Self: TypedNode,
        <Self as TypedNode>::Output: CanString<X> {
}

// Isolation Wildcard
pub struct Wildcard;

pub trait CountArg<'a, DB: Database, X> {
    fn into_count_operand(self) -> Operand<'a, DB>;
}

impl<'a, DB: Database, X> CountArg<'a, DB, X> for Wildcard {
    #[inline(always)]
    fn into_count_operand(self) -> Operand<'a, DB> {
        Operand::from_wildcard()
    }
}

impl<'a, DB: Database, T, X> CountArg<'a, DB, X> for T
    where
        T: AstNode<'a, DB, X> {
    #[inline(always)]
    fn into_count_operand(self) -> Operand<'a, DB> {
        self.into_ast_operand()
    }
}

pub struct CountMarker;

impl<E> AggregateMarker<E> for CountMarker {
    type Output = Static<i64>;
}

impl<'a, DB: Database, E, X> AggregateBuild<'a, DB, E, X> for CountMarker
    where
        E: CountArg<'a, DB, X> {
    #[inline(always)]
    fn build_function(expr: E) -> Function<'a, DB> {
        Function::from(StandardFunction::Count(Box::new(expr.into_count_operand())))
    }
}

// Numeric Aggregation Functions (Avg, Sum): require E::Output: CanMath
macro_rules! impl_numeric_aggregate {
    ($marker:ident, $variant:ident) => {
        pub struct $marker;

        impl<E: TypedNode> AggregateMarker<E> for $marker {
            type Output = E::Output;
        }

        impl<'a, DB: Database, E, X> AggregateBuild<'a, DB, E, X> for $marker
            where
                E: AstNode<'a, DB, X>,
                E::Output: CanMath<X> {
            #[inline(always)]
            fn build_function(expr: E) -> Function<'a, DB> {
                Function::from(StandardFunction::$variant(Box::new(expr.into_ast_operand())))
            }
        }
    };
}

impl_numeric_aggregate!(AvgMarker, Avg);
impl_numeric_aggregate!(SumMarker, Sum);

// Polymorphic Comparison Aggregation Functions (Max, Min)
macro_rules! impl_ord_aggregate {
    ($marker:ident, $variant:ident) => {
        pub struct $marker;

        impl<E: TypedNode> AggregateMarker<E> for $marker {
            type Output = E::Output;
        }

        impl<'a, DB: Database, E, X> AggregateBuild<'a, DB, E, X> for $marker
            where
                E: AstNode<'a, DB, X> {
            #[inline(always)]
            fn build_function(expr: E) -> Function<'a, DB> {
                Function::from(StandardFunction::$variant(Box::new(expr.into_ast_operand())))
            }
        }
    };
}

impl_ord_aggregate!(MaxMarker, Max);
impl_ord_aggregate!(MinMarker, Min);
