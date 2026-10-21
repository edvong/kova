use crate::dsl::ExtFunction;
use crate::dsl::node::Node;
use crate::types::Sqlite;
use kova_core::dsl::func::Function;
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, CanGroup, CanMath, CanOrder, CanString, Operand, Static,
    TypedNode,
};
use std::marker::PhantomData;

// ==========================================
// 1. Bm25 Relevance Scoring
// ==========================================
pub type Bm25<'a, X> = Node<InnerBm25<'a, X>>;

pub struct InnerBm25<'a, X> {
    pub(crate) table: &'static str,
    pub(crate) weights: Vec<f64>,
    pub(crate) _marker: PhantomData<&'a X>,
}

impl<'a, X> Node<InnerBm25<'a, X>> {
    #[inline(always)]
    pub fn weights(mut self, weights: impl Into<Vec<f64>>) -> Self {
        self.0.weights = weights.into();
        self
    }
}

impl<'a, X> TypedNode for InnerBm25<'a, X> {
    type Output = Static<f64>;
}

impl<'a, X> AstNode<'a, Sqlite, X> for InnerBm25<'a, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(ExtFunction::Bm25(self.table, self.weights)))
    }
}

impl<'a, X> CanAlias for InnerBm25<'a, X> {}
impl<'a, X> CanCompare for InnerBm25<'a, X> {}
impl<'a, X> CanOrder for InnerBm25<'a, X> {}
impl<'a, X> CanMath<X> for InnerBm25<'a, X> {}

// ==========================================
// 2. Highlight
// ==========================================
pub type Highlight<'a, X> = Node<InnerHighlight<'a, X>>;

pub struct InnerHighlight<'a, X> {
    pub(crate) table: &'static str,
    pub(crate) col_index: i32,
    pub(crate) left: Operand<'a, Sqlite>,
    pub(crate) right: Operand<'a, Sqlite>,
    pub(crate) _marker: PhantomData<&'a X>,
}

impl<'a, X> TypedNode for InnerHighlight<'a, X> {
    type Output = Static<String>;
}

impl<'a, X> AstNode<'a, Sqlite, X> for InnerHighlight<'a, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(ExtFunction::Highlight(
            self.table,
            self.col_index,
            Box::new(self.left),
            Box::new(self.right),
        )))
    }
}

impl<'a, X> CanAlias for InnerHighlight<'a, X> {}
impl<'a, X> CanCompare for InnerHighlight<'a, X> {}
impl<'a, X> CanOrder for InnerHighlight<'a, X> {}
impl<'a, X> CanGroup for InnerHighlight<'a, X> {}
impl<'a, X> CanString<X> for InnerHighlight<'a, X> {}
// impl<'a, X> CanSqliteString<X> for InnerHighlight<'a, X> {}

// ==========================================
// 3. Snippet
// ==========================================
pub type Snippet<'a, X> = Node<InnerSnippet<'a, X>>;

pub struct InnerSnippet<'a, X> {
    pub(crate) table: &'static str,
    pub(crate) col_index: i32,
    pub(crate) left: Operand<'a, Sqlite>,
    pub(crate) right: Operand<'a, Sqlite>,
    pub(crate) ellipsis: Operand<'a, Sqlite>,
    pub(crate) max_tokens: i32,
    pub(crate) _marker: PhantomData<&'a X>,
}

impl<'a, X> TypedNode for InnerSnippet<'a, X> {
    type Output = Static<String>;
}

impl<'a, X> AstNode<'a, Sqlite, X> for InnerSnippet<'a, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_function(Function::from(ExtFunction::Snippet(
            self.table,
            self.col_index,
            Box::new(self.left),
            Box::new(self.right),
            Box::new(self.ellipsis),
            self.max_tokens,
        )))
    }
}

impl<'a, X> CanAlias for InnerSnippet<'a, X> {}
impl<'a, X> CanCompare for InnerSnippet<'a, X> {}
impl<'a, X> CanOrder for InnerSnippet<'a, X> {}
impl<'a, X> CanGroup for InnerSnippet<'a, X> {}
impl<'a, X> CanString<X> for InnerSnippet<'a, X> {}
