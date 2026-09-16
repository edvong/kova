use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::Function as CoreFunction;
use kova_core::dsl::{
    EraseX, IntoAlias, IntoArg, Projection, ProjectionExpr, SingleArgMarker, WithComponent,
};

// Only accept single projection
pub struct JsonbAgg<T>(pub T);

impl<T> SingleArgMarker for JsonbAgg<T> {}

impl<'a, T, X> IntoArg<Projection<'a, Postgres, X>> for JsonbAgg<T>
    where
        T: IntoArg<Projection<'a, Postgres, X>>, // Only accept single projection
{
    fn into_arg(self) -> Projection<'a, Postgres, X> {
        let inner_proj = self.0.into_arg().erase_x();
        let func = CoreFunction::from_ext(ExtFunction::JsonbAgg(Box::new(inner_proj)));
        Projection::new(ProjectionExpr::Function(func), None)
    }
}

impl<'a, T> JsonbAgg<T> {
    pub fn alias<I: IntoAlias<A>, A, X>(self, alias: I) -> Projection<'a, Postgres, X>
        where
            T: IntoArg<Projection<'a, Postgres, X>>, {
        let mut proj: Projection<'a, Postgres, X> = self.into_arg();
        proj.alias(alias.into_identifier());
        proj
    }
}
