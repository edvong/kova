use crate::dsl::function::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    EraseX, IntoArg, IntoIdentifier, Projection, ProjectionExpr, SingleArgMarker, WithComponent,
};

// group_concat(X, Y)
pub struct GroupConcat<T> {
    pub expr: T,
    pub separator: Option<&'static str>,
}

impl<T> SingleArgMarker for GroupConcat<T> {}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for GroupConcat<T>
    where
        T: IntoArg<Projection<'a, Sqlite, X>> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        let proj = self.expr.into_arg().erase_x();
        let func = Function::from_ext(ExtFunction::GroupConcat(Box::new(proj), self.separator));
        Projection::new(ProjectionExpr::Function(func), None)
    }
}

impl<'a, T> GroupConcat<T> {
    pub fn alias<I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Sqlite, X>
        where
            T: IntoArg<Projection<'a, Sqlite, X>> {
        let mut proj: Projection<'a, Sqlite, X> = self.into_arg();
        proj.alias(alias);
        proj
    }
}
