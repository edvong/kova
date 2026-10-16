#![allow(non_snake_case)]

use crate::dsl::ExtFunction;
use crate::dsl::node::{CanPgJsonb, Node};
use crate::types::Pg;
use kova_core::dsl::func::Function;
use kova_core::dsl::{AstNode, CanAlias, CanCompare, JsonbType, Operand, Static, TypedNode};

pub type JsonbObject<K, V> = Node<InnerJsonbObject<K, V>>;

pub const fn JsonbObject<K, V>(keys: K, values: V) -> JsonbObject<K, V> {
    Node(InnerJsonbObject { keys, values: Some(values) })
}

pub const fn JsonbObjectSingle<K>(keys: K) -> JsonbObject<K, K> {
    Node(InnerJsonbObject { keys, values: None })
}

pub struct InnerJsonbObject<K, V> {
    keys: K,
    values: Option<V>,
}

impl<K: TypedNode, V: TypedNode> TypedNode for InnerJsonbObject<K, V> {
    type Output = Static<JsonbType>;
}

impl<K, V> CanAlias for InnerJsonbObject<K, V> {}
impl<K, V> CanCompare for InnerJsonbObject<K, V> {}
impl<K, V> CanPgJsonb for InnerJsonbObject<K, V> {}

impl<'a, K, V, X> AstNode<'a, Pg, X> for InnerJsonbObject<K, V>
    where
        K: TypedNode + AstNode<'a, Pg, X>,
        V: TypedNode + AstNode<'a, Pg, X> {
    fn into_ast_operand(self) -> Operand<'a, Pg> {
        Operand::from_function(Function::from(ExtFunction::JsonbObject {
            keys: Box::new(self.keys.into_ast_operand()),
            values: self.values.map(|values| Box::new(values.into_ast_operand())),
        }))
    }
}
