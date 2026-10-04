use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{IntoArg, IntoIdentifier, IntoOperand, IntoProjection, OpAst, OpNode, Operand, Projection, SingleArgMarker};

/// Maps to SQLite's native `json(X)` function.
/// Note to self: Use this during the final projection (Select) phase. It forcibly converts SQLite's
/// internal binary JSONB format back into standard JSON Text, so Rust can actually parse it.
pub struct Json<T>(pub T);

impl<T> Json<T> {
    pub fn alias<'a, I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Sqlite, X>
        where
            T: IntoOperand<'a, T, Sqlite, OpAst, X> {
        self.into_projection().alias(alias)
    }
}

impl<T> SingleArgMarker for Json<T> {}

impl<T: OpNode> OpNode for Json<T> {
    type Input = T::Output;
    type Output = String;
}

impl<'a, Target, T, X> IntoOperand<'a, Target, Sqlite, OpAst, X> for Json<T>
    where
        T: IntoOperand<'a, T, Sqlite, OpAst, X> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(ExtFunction::Json(Box::new(
            self.0.into_operand(),
        ))))
    }
}

impl<'a, T, X> IntoProjection<'a, Sqlite, X> for Json<T>
    where
        T: IntoOperand<'a, T, Sqlite, OpAst, X> {
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(IntoOperand::<T, Sqlite, OpAst, X>::into_operand(self))
    }
}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for Json<T>
    where
        Self: IntoProjection<'a, Sqlite, X> {
    // Rule: As long as the wrapped inner item is capable of being projected, Json can wrap it.
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}
