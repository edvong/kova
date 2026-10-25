#![allow(non_snake_case)]

use crate::dsl::node::Node;
use crate::dsl::ExtFunction;
use crate::types::Sqlite;
use kova_core::dsl::func::{
    Aggregate, AggregateBuild, AggregateMarker, Function, Quantifier, SetDistinct, SetFilter,
    SetOrderBy, SetWindow, Window,
};
use kova_core::dsl::{
    AstNode, CanAlias, CanCompare, CanDistinct, CanFilter, CanOrder, CanOrderBy, CanString,
    CanWindow, Condition, Operand, Ordering, Static, TypedNode,
};
use std::marker::PhantomData;

pub struct JsonGroupArrayMarker;
pub struct JsonbGroupArrayMarker;

pub type JsonGroupArray<'a, T, X> = Node<Aggregate<'a, Sqlite, JsonGroupArrayMarker, T, X>>;
pub type JsonbGroupArray<'a, T, X> = Node<Aggregate<'a, Sqlite, JsonbGroupArrayMarker, T, X>>;

pub const fn JsonGroupArray<'a, T, X>(target: T) -> JsonGroupArray<'a, T, X> {
    Node(Aggregate::new(target))
}

pub const fn JsonbGroupArray<'a, T, X>(target: T) -> JsonbGroupArray<'a, T, X> {
    Node(Aggregate::new(target))
}

impl<T: TypedNode> AggregateMarker<T> for JsonGroupArrayMarker {
    type Output = Static<String>;
}

impl<'a, T, X> AggregateBuild<'a, Sqlite, T, X> for JsonGroupArrayMarker
    where
        T: AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn build_function(expr: T) -> Function<'a, Sqlite> {
        // Function::agg("json_group_array", expr.into_ast_operand())
        Function::from(ExtFunction::JsonGroupArray(Box::new(expr.into_ast_operand())))
    }
}

impl<T: TypedNode> AggregateMarker<T> for JsonbGroupArrayMarker {
    type Output = Static<Vec<u8>>;
}

impl<'a, T, X> AggregateBuild<'a, Sqlite, T, X> for JsonbGroupArrayMarker
    where
        T: AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn build_function(expr: T) -> Function<'a, Sqlite> {
        // Function::agg("jsonb_group_array", expr.into_ast_operand())
        Function::from(ExtFunction::JsonbGroupArray(Box::new(expr.into_ast_operand())))
    }
}

pub type GroupConcat<'a, T, X> = Node<InnerGroupConcat<'a, T, X>>;

pub const fn GroupConcat<'a, T, X>(operand: T) -> GroupConcat<'a, T, X> {
    Node(InnerGroupConcat {
        operand,
        separator: None,
        quantifier: None,
        order_by: Vec::new(),
        filter: None,
        window: None,
        _marker: PhantomData,
    })
}

pub struct InnerGroupConcat<'a, T, X> {
    operand: T,
    separator: Option<&'static str>,
    quantifier: Option<Quantifier>,
    order_by: Vec<Ordering<'a, Sqlite, ()>>,
    filter: Option<Condition<'a, Sqlite, ()>>,
    window: Option<Window<'a, Sqlite, ()>>,
    _marker: PhantomData<X>,
}

impl<'a, T, X> Node<InnerGroupConcat<'a, T, X>> {
    #[inline(always)]
    pub fn separator(mut self, s: &'static str) -> Self {
        self.0.separator = Some(s);
        self
    }
}

impl<'a, T, X> TypedNode for InnerGroupConcat<'a, T, X> {
    type Output = Static<String>;
}

impl<'a, T, X> AstNode<'a, Sqlite, X> for InnerGroupConcat<'a, T, X>
    where
        T: AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Sqlite> {
        let mut f = Function::from(ExtFunction::GroupConcat(
            Box::new(self.operand.into_ast_operand()),
            self.separator,
        ));
        if let Some(q) = self.quantifier {
            f = f.quantifier(q);
        }
        if !self.order_by.is_empty() {
            f = f.order_by(self.order_by);
        }
        if let Some(filter) = self.filter {
            f = f.filter(filter);
        }
        if let Some(window) = self.window {
            f = f.window(window);
        }
        Operand::from_function(f)
    }
}

impl<'a, T, X> SetDistinct for InnerGroupConcat<'a, T, X> {
    #[inline(always)]
    fn set_distinct(mut self, quantifier: Quantifier) -> Self {
        self.quantifier = Some(quantifier);
        self
    }
}

impl<'a, T, X> SetOrderBy<'a, Sqlite, X> for InnerGroupConcat<'a, T, X> {
    #[inline(always)]
    fn set_order_by(mut self, ordering: Vec<Ordering<'a, Sqlite, ()>>) -> Self {
        self.order_by = ordering;
        self
    }
}

impl<'a, T, X> SetFilter<'a, Sqlite, X> for InnerGroupConcat<'a, T, X> {
    #[inline(always)]
    fn set_filter(mut self, cond: Condition<'a, Sqlite, ()>) -> Self {
        self.filter = Some(cond);
        self
    }
}

impl<'a, T, X> SetWindow<'a, Sqlite, X> for InnerGroupConcat<'a, T, X> {
    #[inline(always)]
    fn set_window(mut self, window: Window<'a, Sqlite, ()>) -> Self {
        self.window = Some(window);
        self
    }
}

impl<'a, T, X> CanAlias for InnerGroupConcat<'a, T, X> {}
impl<'a, T, X> CanCompare for InnerGroupConcat<'a, T, X> {}
impl<'a, T, X> CanOrder for InnerGroupConcat<'a, T, X> {}
impl<'a, T, X> CanString<X> for InnerGroupConcat<'a, T, X> {}
impl<'a, T, X> CanDistinct for InnerGroupConcat<'a, T, X> {}
impl<'a, T, X> CanFilter for InnerGroupConcat<'a, T, X> {}
impl<'a, T, X> CanOrderBy for InnerGroupConcat<'a, T, X> {}
impl<'a, T, X> CanWindow for InnerGroupConcat<'a, T, X> {}
