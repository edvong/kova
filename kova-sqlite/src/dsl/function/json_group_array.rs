use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{IntoArg, IntoIdentifier, IntoOperand, IntoProjection, OpAst, OpNode, Operand, Projection, SingleArgMarker};

// json_group_array(X)
pub struct JsonGroupArray<T>(pub T);

impl<'a, T> JsonGroupArray<T> {
    pub fn alias<I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Sqlite, X>
    where
        Self: IntoProjection<'a, Sqlite, X> {
        self.into_projection().alias(alias)
    }
}

impl<T> SingleArgMarker for JsonGroupArray<T> {}

impl<T: OpNode> OpNode for JsonGroupArray<T> {
    type Input = T::Output;
    type Output = Vec<u8>;
}

impl<'a, Target, T, X> IntoOperand<'a, Target, Sqlite, OpAst, X> for JsonGroupArray<T>
    where
        T: OpNode + IntoOperand<'a, <T as OpNode>::Output, Sqlite, OpAst, X> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(ExtFunction::JsonGroupArray(Box::new(
            self.0.into_operand(),
        ))))
    }
}

impl<'a, T, X> IntoProjection<'a, Sqlite, X> for JsonGroupArray<T>
    where
        Self: IntoOperand<'a, (), Sqlite, OpAst, X> {
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(self.into_operand())
    }
}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for JsonGroupArray<T>
    where
        Self: IntoProjection<'a, Sqlite, X> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}
