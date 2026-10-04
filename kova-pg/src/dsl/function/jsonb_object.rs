use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::Function;
use kova_core::dsl::{EraseX, IntoArg, IntoArgs, IntoIdentifier, IntoOperand, IntoProjection, OpAst, Operand, Projection, SingleArgMarker};

// Accept multiple projections
pub struct JsonbObject<T>(pub T);

impl<'a, T> JsonbObject<T> {
    pub fn alias<I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Postgres, X>
        where
            Self: IntoProjection<'a, Postgres, X> {
        self.into_projection().alias(alias)
    }
}

impl<T> SingleArgMarker for JsonbObject<T> {}

impl<'a, Target, T, X> IntoOperand<'a, Target, Postgres, OpAst, X> for JsonbObject<T>
    where
        T: IntoArgs<Projection<'a, Postgres, X>> {
    fn into_operand(self) -> Operand<'a, Postgres> {
        Operand::from_function(Function::from(ExtFunction::JsonbObject(
            self.0.into_mapped_args(|p| p.erase_x())
        )))
    }
}

impl<'a, T, X> IntoProjection<'a, Postgres, X> for JsonbObject<T>
    where
        Self: IntoOperand<'a, T, Postgres, OpAst, X> {
    fn into_projection(self) -> Projection<'a, Postgres, X> {
        Projection::new(self.into_operand())
    }
}

// swallowing polymorphic tuples
impl<'a, T, X> IntoArg<Projection<'a, Postgres, X>> for JsonbObject<T>
    where
        Self: IntoProjection<'a, Postgres, X>, {
    fn into_arg(self) -> Projection<'a, Postgres, X> {
        self.into_projection()
    }
}
