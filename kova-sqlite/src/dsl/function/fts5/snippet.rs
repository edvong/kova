use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    BuildProjection, IntoArg, IntoIdentifier, IntoOperand, Operand, Projection, ProjectionExpr,
    SingleArgMarker, WithComponent,
};
use std::marker::PhantomData;

// Search Result Snippet
pub struct Snippet<'a, X> {
    pub(crate) table: &'static str,
    pub(crate) col_index: i32,
    pub(crate) left: Operand<'a, Sqlite>,
    pub(crate) right: Operand<'a, Sqlite>,
    pub(crate) ellipsis: Operand<'a, Sqlite>,
    pub(crate) max_tokens: i32,
    pub(crate) _marker: PhantomData<&'a X>,
}

impl<'a, X> Snippet<'a, X> {
    pub fn new<L, R, E, K1, K2, K3>(
        table: &'static str, col_index: i32, left: L, right: R, ellipsis: E, max_tokens: i32,
    ) -> Self
        where
            L: IntoOperand<'a, String, Sqlite, K1, X>,
            R: IntoOperand<'a, String, Sqlite, K2, X>,
            E: IntoOperand<'a, String, Sqlite, K3, X> {
        Self {
            table,
            col_index,
            left: left.into_operand(),
            right: right.into_operand(),
            ellipsis: ellipsis.into_operand(),
            max_tokens,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    fn into_func(self) -> Function<'a, Sqlite> {
        Function::from_ext(ExtFunction::Snippet(
            self.table,
            self.col_index,
            Box::new(self.left),
            Box::new(self.right),
            Box::new(self.ellipsis),
            self.max_tokens,
        ))
    }

    pub fn alias<I: IntoIdentifier<A>, A>(self, alias: I) -> Projection<'a, Sqlite, X> {
        self.build_projection(alias)
    }
}

impl<'a, X> BuildProjection<'a, Sqlite, X> for Snippet<'a, X> {
    #[inline(always)]
    fn build_projection<A, U>(self, alias: A) -> Projection<'a, Sqlite, X>
        where
            A: IntoIdentifier<U> {
        Projection::new(
            ProjectionExpr::Function(self.into_func()),
            Some(alias.into_identifier().meta),
        )
    }
}

impl<'a, X> SingleArgMarker for Snippet<'a, X> {}

impl<'a, X> IntoArg<Projection<'a, Sqlite, X>> for Snippet<'a, X> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        Projection::new(ProjectionExpr::Function(self.into_func()), None)
    }
}
