use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    IntoArg, IntoIdentifier, Operand, Projection, ProjectionExpr, SingleArgMarker, WithComponent,
};

/// Maps to SQLite's native `json(X)` function.
/// Note to self: Use this during the final projection (Select) phase. It forcibly converts SQLite's
/// internal binary JSONB format back into standard JSON Text, so Rust can actually parse it.
pub struct Json<T>(pub T);

impl<T> SingleArgMarker for Json<T> {}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for Json<T>
    where
        T: IntoArg<Projection<'a, Sqlite, X>> {
    // Rule: As long as the wrapped inner item is capable of being projected, Json can wrap it.
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        let proj = self.0.into_arg();
        proj.transform_expr(|expr| {
            // Peel back the wrapper and convert the inner expression into a generic Operand.
            let operand = match expr {
                ProjectionExpr::Column(meta) => Operand::from_identifier(meta),
                ProjectionExpr::Function(func) => Operand::from_function(func),
                ProjectionExpr::Subselect(stmt) => Operand::from_subselect(*stmt),
            };
            // Brute-force wrap it in a native json(...) call.
            let json_func = Function::from_ext(ExtFunction::Json(Box::new(operand)));
            // Overwrite the original expression with this newly wrapped function,
            // while making absolutely sure we preserve any Alias it might have already carried.
            ProjectionExpr::Function(json_func)
        })
    }
}

impl<T> Json<T> {
    pub fn alias<'a, I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Sqlite, X>
        where
            Self: IntoArg<Projection<'a, Sqlite, X>> {
        let mut proj = self.into_arg();
        proj.alias(alias);
        proj
    }
}
