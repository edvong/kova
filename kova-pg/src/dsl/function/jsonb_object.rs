use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::Function as CoreFunction;
use kova_core::dsl::{
    EraseX, IntoIdentifier, IntoArg, IntoArgs, Projection, ProjectionExpr, SingleArgMarker,
    WithComponent,
};

pub struct JsonbObject<T>(pub T);

impl<T> SingleArgMarker for JsonbObject<T> {}

// swallowing polymorphic tuples
impl<'a, T, X> IntoArg<Projection<'a, Postgres, X>> for JsonbObject<T>
    where
        T: IntoArgs<Projection<'a, Postgres, X>> {
    fn into_arg(self) -> Projection<'a, Postgres, X> {
        let projs = self.0.into_mapped_args(|p| p.erase_x());
        let func = CoreFunction::from_ext(ExtFunction::JsonbObject(projs));
        Projection::new(ProjectionExpr::Function(func), None)
    }
}

impl<'a, T> JsonbObject<T> {
    pub fn alias<I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Postgres, X>
        where
            T: IntoArgs<Projection<'a, Postgres, X>> {
        let mut proj: Projection<'a, Postgres, X> = self.into_arg();
        proj.alias(alias.into_identifier());
        proj
    }
}
