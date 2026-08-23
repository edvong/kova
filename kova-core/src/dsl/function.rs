use crate::dsl::projection::{IntoProjection, Projectable, Projection, StandardProjection};
use crate::dsl::{Column, Component};
use crate::types::{ColumnMeta, Database, Extension};
use std::marker::PhantomData;

pub type Function<'a, DB> = Component<
    StandardFunction<'a, DB>,
    <<DB as Database>::Extension<'a> as Extension<'a>>::Function<'a>,
>;

pub enum StandardFunction<'a, DB: Database> {
    CountAll,
    Count(ColumnMeta, Quantifier),
    Sum(ColumnMeta, Quantifier),
    Max(ColumnMeta, Quantifier),
    Min(ColumnMeta, Quantifier),
    Avg(ColumnMeta, Quantifier),

    #[doc(hidden)]
    _Marker(PhantomData<&'a DB>),
}

pub struct CountAll;
pub struct Count<T>(pub T);
pub struct Sum<T>(pub T);
pub struct Max<T>(pub T);
pub struct Min<T>(pub T);
pub struct Avg<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    All,
    Distinct,
}

// ==== Function to Projection ====

impl<'a, DB: Database> Function<'a, DB> {
    pub fn alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
        Component::Standard(StandardProjection::Function(self, Some(col.meta)))
    }
}

impl<'a, DB: Database> IntoProjection<'a, DB> for Function<'a, DB> {
    fn into_projection(self) -> Projection<'a, DB> {
        Component::Standard(StandardProjection::Function(self, None))
    }
}

impl<'a, DB: Database> Projectable<'a, DB> for Function<'a, DB> {
    fn into_projections(self) -> Vec<Projection<'a, DB>> {
        vec![self.into_projection()]
    }
}

// ==== Function to Projection ====

// No Arguments, independent implementation
impl CountAll {
    pub fn alias<U, DB: Database>(self, col: Column<U, DB>) -> Projection<DB> {
        let func = Component::Standard(StandardFunction::CountAll);
        Component::Standard(StandardProjection::Function(func, Some(col.meta)))
    }
}

impl<'a, DB: Database> IntoProjection<'a, DB> for CountAll {
    fn into_projection(self) -> Projection<'a, DB> {
        let func = Component::Standard(StandardFunction::CountAll);
        Component::Standard(StandardProjection::Function(func, None))
    }
}

impl<'a, DB: Database> Projectable<'a, DB> for CountAll {
    fn into_projections(self) -> Vec<Projection<'a, DB>> {
        vec![self.into_projection()]
    }
}

macro_rules! impl_aggregate_function {
    ($StructName:ident, $Variant:ident) => {
        impl<'a, T, DB: Database> $StructName<Column<'a, T, DB>> {
            pub fn distinct(self) -> Function<'a, DB> {
                Component::Standard(StandardFunction::$Variant(
                    self.0.meta,
                    Quantifier::Distinct,
                ))
            }

            pub fn alias<U>(self, col: Column<'a, U, DB>) -> Projection<'a, DB> {
                let func =
                    Component::Standard(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Component::Standard(StandardProjection::Function(func, Some(col.meta)))
            }
        }

        impl<'a, T, DB: Database> IntoProjection<'a, DB> for $StructName<Column<'a, T, DB>> {
            fn into_projection(self) -> Projection<'a, DB> {
                let func =
                    Component::Standard(StandardFunction::$Variant(self.0.meta, Quantifier::All));
                Component::Standard(StandardProjection::Function(func, None))
            }
        }

        impl<'a, T, DB: Database> Projectable<'a, DB> for $StructName<Column<'a, T, DB>> {
            fn into_projections(self) -> Vec<Projection<'a, DB>> {
                vec![self.into_projection()]
            }
        }
    };
}

impl_aggregate_function!(Count, Count);
impl_aggregate_function!(Sum, Sum);
impl_aggregate_function!(Max, Max);
impl_aggregate_function!(Min, Min);
impl_aggregate_function!(Avg, Avg);
