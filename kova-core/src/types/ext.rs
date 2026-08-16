use crate::dsl::ExtensionOperator;

pub trait Extension<'a>: Sized {
    type Column<T>;
    type Operator<'o> : ExtensionOperator;
}