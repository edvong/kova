use crate::dsl::function::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    EraseX, IntoArg, IntoIdentifier, Projection, ProjectionExpr, SingleArgMarker, WithComponent,
};

pub struct GroupConcat<T>(pub T);

pub struct GroupConcatState<T> {
    pub(crate) expr: T,
    pub(crate) separator: Option<&'static str>,
}

impl<T> SingleArgMarker for GroupConcat<T> {}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for GroupConcat<T>
    where
        T: IntoArg<Projection<'a, Sqlite, X>> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        let proj = self.0.into_arg().erase_x();
        let func = Function::from_ext(ExtFunction::GroupConcat(Box::new(proj), None));
        Projection::new(ProjectionExpr::Function(func), None)
    }
}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for GroupConcat<GroupConcatState<T>>
    where
        T: IntoArg<Projection<'a, Sqlite, X>> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        let proj = self.0.expr.into_arg().erase_x();
        let func = Function::from_ext(ExtFunction::GroupConcat(Box::new(proj), self.0.separator));
        Projection::new(ProjectionExpr::Function(func), None)
    }
}

impl<T> GroupConcat<T> {
    pub fn separator(self, separator: &'static str) -> GroupConcat<GroupConcatState<T>> {
        GroupConcat(GroupConcatState { expr: self.0, separator: Some(separator) })
    }

    pub fn alias<'a, I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Sqlite, X>
        where
            Self: IntoArg<Projection<'a, Sqlite, X>> {
        let mut proj: Projection<'a, Sqlite, X> = self.into_arg();
        proj.alias(alias.into_identifier());
        proj
    }
}
