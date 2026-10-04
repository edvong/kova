use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{EraseX, IntoArg, IntoArgs, IntoIdentifier, IntoOperand, IntoProjection, OpAst, OpNode, Operand, Projection, SingleArgMarker};

pub struct JsonObject<T>(pub T);

impl<'a, T> JsonObject<T> {
    pub fn alias<I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Sqlite, X>
        where
            Self: IntoProjection<'a, Sqlite, X> {
        self.into_projection().alias(alias)
    }
}

impl<T> SingleArgMarker for JsonObject<T> {}

impl<T> OpNode for JsonObject<T> {
    type Input = ();
    type Output = String;
}

impl<'a, Target, T, X> IntoOperand<'a, Target, Sqlite, OpAst, X> for JsonObject<T>
    where
        T: IntoArgs<Projection<'a, Sqlite, X>> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(ExtFunction::JsonObject(
            self.0.into_mapped_args(|p| p.erase_x())
        )))
    }
}

impl<'a, T, X> IntoProjection<'a, Sqlite, X> for JsonObject<T>
    where
        Self: IntoOperand<'a, (), Sqlite, OpAst, X> {
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(self.into_operand())
    }
}


impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for JsonObject<T>
    where
        Self: IntoProjection<'a, Sqlite, X> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}
