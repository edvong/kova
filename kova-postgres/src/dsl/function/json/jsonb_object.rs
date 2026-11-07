#![allow(non_snake_case)]

use crate::dsl::can::CanPgArray;
use crate::dsl::node::Node;
use crate::dsl::CanPgJsonb;
use crate::dsl::func::ExtFunction;
use crate::types::Postgres;
use kova_kernel::dsl::func::Function;
use kova_kernel::dsl::{AstNode, CanAlias, CanCompare, JsonbType, Operand, Static, TypedNode};

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
impl<K, V, X> CanPgJsonb<X> for InnerJsonbObject<K, V> {}

impl<'a, K, V, X> AstNode<'a, Postgres, X> for InnerJsonbObject<K, V>
    where
        K: TypedNode + AstNode<'a, Postgres, X>,
        V: TypedNode + AstNode<'a, Postgres, X>,
        K::Output: CanPgArray<X>,
        V::Output: CanPgArray<X> {
    fn into_ast_operand(self) -> Operand<'a, Postgres> {
        Operand::from_function(Function::from(ExtFunction::JsonbObject {
            keys: Box::new(self.keys.into_ast_operand()),
            values: self.values.map(|values| Box::new(values.into_ast_operand())),
        }))
    }
}
