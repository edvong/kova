use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    EraseX, IntoArg, IntoIdentifier, Projection, ProjectionExpr, SingleArgMarker, WithComponent,
};

// json_group_array(X)
pub struct JsonGroupArray<T>(pub T);

impl<T> SingleArgMarker for JsonGroupArray<T> {}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for JsonGroupArray<T>
    where
        T: IntoArg<Projection<'a, Sqlite, X>> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        let proj = self.0.into_arg().erase_x();
        let func = Function::from_ext(ExtFunction::JsonGroupArray(Box::new(proj)));
        Projection::new(ProjectionExpr::Function(func), None)
    }
}

impl<'a, T> JsonGroupArray<T> {
    pub fn alias<I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Sqlite, X>
        where
            T: IntoArg<Projection<'a, Sqlite, X>> {
        let mut proj: Projection<'a, Sqlite, X> = self.into_arg();
        proj.alias(alias.into_identifier());
        proj
    }
}
