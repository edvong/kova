mod json_each;
mod json_tree;
mod srf_node;

pub use self::json_each::JsonEach;
pub use self::json_tree::JsonTree;

use crate::dsl::can::CanSqliteJson;
use crate::dsl::node::Node;
use crate::types::Sqlite;
use kova_core::dsl::{AstNode, Compatible, Operand, Static};
use kova_core::types::Encode;

pub enum ExtSrfFunction<'a> {
    JsonEach(Operand<'a, Sqlite>, Option<Operand<'a, Sqlite>>),
    JsonTree(Operand<'a, Sqlite>, Option<Operand<'a, Sqlite>>),
}

pub struct JsonSrfArgs<'a> {
    pub base: Operand<'a, Sqlite>,
    pub path: Option<Operand<'a, Sqlite>>,
}

pub trait IntoSrfFunction<'a, X> {
    fn into_srf_function(self) -> JsonSrfArgs<'a>;
}

impl<'a, T, X> IntoSrfFunction<'a, X> for Node<T>
    where
        T: CanSqliteJson<X> + AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn into_srf_function(self) -> JsonSrfArgs<'a> {
        JsonSrfArgs { base: self.0.into_ast_operand(), path: None }
    }
}

impl<'a, T, P, X> IntoSrfFunction<'a, X> for (Node<T>, P)
    where
        T: CanSqliteJson<X> + AstNode<'a, Sqlite, X>,
        Sqlite: Encode<'a, P, X>,
        Static<P>: Compatible<Static<String>, X> {
    #[inline(always)]
    fn into_srf_function(self) -> JsonSrfArgs<'a> {
        JsonSrfArgs {
            base: self.0.0.into_ast_operand(),
            path: Some(Operand::from_result(<Sqlite as Encode<'a, P, X>>::encode(self.1))),
        }
    }
}
