use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    IntoArg, IntoIdentifier, IntoOperand, IntoProjection, OpAst, Operand, Projection,
    SingleArgMarker,
};

// Only accept single projection
pub struct JsonbAgg<T>(pub T);

impl<'a, T> JsonbAgg<T> {
    pub fn alias<I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Postgres, X>
        where
            T: IntoOperand<'a, T, Postgres, OpAst, X> {
        self.into_projection().alias(alias)
    }
}

impl<T> SingleArgMarker for JsonbAgg<T> {}

impl<'a, T, X> IntoProjection<'a, Postgres, X> for JsonbAgg<T>
    where
        T: IntoOperand<'a, T, Postgres, OpAst, X> {
    fn into_projection(self) -> Projection<'a, Postgres, X> {
        Projection::new(Operand::from_function(Function::from(ExtFunction::JsonbAgg(
            Box::new(self.0.into_operand()),
        ))))
    }
}

// Only accept single projection
impl<'a, T, X> IntoArg<Projection<'a, Postgres, X>> for JsonbAgg<T>
    where
        Self: IntoProjection<'a, Postgres, X> {
    fn into_arg(self) -> Projection<'a, Postgres, X> {
        self.into_projection()
    }
}
