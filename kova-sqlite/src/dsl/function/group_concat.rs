use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    IntoArg, IntoIdentifier, IntoOperand, IntoProjection, OpAst, OpNode, Operand, Projection,
    SingleArgMarker,
};

pub struct GroupConcat<T> {
    pub(crate) operand: T,
    pub(crate) separator: Option<&'static str>,
}

#[allow(non_snake_case)]
pub const fn GroupConcat<T>(operand: T) -> GroupConcat<T> {
    GroupConcat { operand, separator: None }
}

impl<T> SingleArgMarker for GroupConcat<T> {}

impl<T> OpNode for GroupConcat<T> {
    type Input = ();
    type Output = String;
}

impl<T> GroupConcat<T> {
    pub fn separator(mut self, s: &'static str) -> Self {
        self.separator = Some(s);
        self
    }

    pub fn alias<'a, I: IntoIdentifier<A>, A, X>(self, alias: I) -> Projection<'a, Sqlite, X>
        where
            Self: IntoProjection<'a, Sqlite, X> {
        self.into_projection().alias(alias)
    }
}


impl<'a, Target, T, X> IntoOperand<'a, Target, Sqlite, OpAst, X> for GroupConcat<T>
    where
        T: OpNode + IntoOperand<'a, <T as OpNode>::Output, Sqlite, OpAst, X> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(
            ExtFunction::GroupConcat(Box::new(self.operand.into_operand()), self.separator)
        ))
    }
}

impl<'a, T, X> IntoProjection<'a, Sqlite, X> for GroupConcat<T>
    where
        Self: IntoOperand<'a, (), Sqlite, OpAst, X> {
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(self.into_operand())
    }
}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>> for GroupConcat<T>
    where
        Self: IntoProjection<'a, Sqlite, X> {
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}
