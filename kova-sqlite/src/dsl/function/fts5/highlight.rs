use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    IntoArg, IntoIdentifier, IntoOperand, IntoOrdering, IntoProjection, OpAst, Operand, Ordering,
    Projection, SingleArgMarker,
};
use std::marker::PhantomData;

// Hit Highlighting
pub struct Highlight<'a, X> {
    pub(crate) table: &'static str,
    pub(crate) col_index: i32,
    pub(crate) left: Operand<'a, Sqlite>,
    pub(crate) right: Operand<'a, Sqlite>,
    pub(crate) _marker: PhantomData<&'a X>,
}

impl<'a, X> Highlight<'a, X> {
    pub fn new<L, R, K1, K2>(table: &'static str, col_index: i32, left: L, right: R) -> Self
        where
            L: IntoOperand<'a, String, Sqlite, K1, X>,
            R: IntoOperand<'a, String, Sqlite, K2, X> {
        Self {
            table,
            col_index,
            left: left.into_operand(),
            right: right.into_operand(),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    fn into_func(self) -> Function<'a, Sqlite> {
        Function::from(ExtFunction::Highlight(
            self.table,
            self.col_index,
            Box::new(self.left),
            Box::new(self.right),
        ))
    }

    pub fn alias<I: IntoIdentifier<A>, A>(self, alias: I) -> Projection<'a, Sqlite, X> {
        self.into_projection().alias(alias)
    }
}

impl<'a, X> SingleArgMarker for Highlight<'a, X> {}

impl<'a, X> IntoOperand<'a, (), Sqlite, OpAst, X> for Highlight<'a, X> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(self.into_func())
    }
}

impl<'a, X> IntoOrdering<'a, Sqlite, X> for Highlight<'a, X> {
    fn into_ordering(self) -> Ordering<'a, Sqlite, X> {
        Ordering::from_operand(self.into_operand())
    }
}

impl<'a, X> IntoProjection<'a, Sqlite, X> for Highlight<'a, X> {
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(self.into_operand())
    }
}

impl<'a, X> IntoArg<Projection<'a, Sqlite, X>> for Highlight<'a, X> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}
