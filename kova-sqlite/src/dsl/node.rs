#![allow(clippy::wrong_self_convention)]

use std::any::TypeId;
use crate::dsl::can::{CanSqliteFts, CanSqliteJson, CanSqliteString};
use crate::dsl::function::{
    Bm25, Highlight, InnerBm25, InnerHighlight, InnerJsonExtract, InnerSnippet, JsonExtract,
    JsonExtractMarker, JsonbExtract, JsonbExtractMarker, Snippet,
};
use crate::dsl::{ExtAssignment, ExtOperator};
use crate::types::Sqlite;
use kova_core::dsl::func::{
    IntoPartitionBy, PartitionBy, Quantifier, SetDistinct, SetFilter, SetOrderBy, SetWindow, Window,
};
use kova_core::dsl::statement::IntoSelectStatement;
use kova_core::dsl::{
    ArgAst, Assignment, AstNode, CanAlias, CanAssignment, CanCompare, CanConflict, CanDistinct,
    CanFilter, CanGroup, CanMath, CanOrder, CanOrderBy, CanString, CanWindow, Close, Comparator,
    Compatible, Condition, Conflict, Direction, EraseX, GroupBy, Identifier, IntoArg, IntoConflict,
    IntoGroupBy, IntoIdentifier, IntoOperand, IntoOrdering, IntoProjection, Math, MathOperator,
    Operand, Operator, Ordering, Projection, StandardAssignment, StandardOperator, Static,
    TypedNode,
};
use kova_core::types::SetTag;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Node<T>(pub T);

impl<T: CanGroup> CanGroup for Node<T> {}
impl<T: CanConflict> CanConflict for Node<T> {}
impl<T: CanMath<X>, X> CanMath<X> for Node<T> {}
impl<T: CanString<X>, X> CanString<X> for Node<T> {}
impl<T: CanSqliteJson<X>, X> CanSqliteJson<X> for Node<T> {}
impl<T: CanSqliteFts<X>, X> CanSqliteFts<X> for Node<T> {
    #[inline(always)]
    fn fts_table(&self) -> &'static str {
        self.0.fts_table()
    }
}

impl<T: TypedNode> TypedNode for Node<T> {
    type Output = T::Output;
}

impl<'a, T, X> AstNode<'a, Sqlite, X> for Node<T>
    where
        T: AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Sqlite> {
        self.0.into_ast_operand()
    }
}

#[diagnostic::do_not_recommend]
impl<'a, T, Target, X> IntoOperand<'a, Target, Sqlite, Close, X> for Node<T>
    where
        T: AstNode<'a, Sqlite, X>,
        T::Output: Compatible<Target, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Sqlite> {
        self.0.into_ast_operand()
    }
}

impl<T> Node<T> {
    #[inline(always)]
    pub fn tag<Marker: 'static>(mut self) -> Self
        where
            T: SetTag {
        self.0.set_tag(TypeId::of::<Marker>());
        self
    }

    #[inline(always)]
    fn comparator<'a, X>(self) -> Comparator<'a, T::Output, Sqlite>
        where
            T: AstNode<'a, Sqlite, X> {
        Comparator::new(self.0.into_ast_operand())
    }
}

impl<'a, T> Node<T>
    where
        T: CanCompare + TypedNode {
    #[inline(always)]
    pub fn eq<V, K, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Eq)
    }

    #[inline(always)]
    pub fn ne<V, K, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Ne)
    }

    #[inline(always)]
    pub fn gt<V, K, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Gt)
    }

    #[inline(always)]
    pub fn ge<V, K, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Ge)
    }

    #[inline(always)]
    pub fn lt<V, K, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Lt)
    }

    #[inline(always)]
    pub fn le<V, K, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Le)
    }

    #[inline(always)]
    pub fn is_null<X>(self) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X> {
        self.comparator::<X>().unary_op(StandardOperator::IsNull)
    }

    #[inline(always)]
    pub fn is_not_null<X>(self) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X> {
        self.comparator::<X>().unary_op(StandardOperator::IsNotNull)
    }

    #[inline(always)]
    pub fn between<L, U, KL, KU, X>(self, lower: L, upper: U) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            L: IntoOperand<'a, T::Output, Sqlite, KL, X>,
            U: IntoOperand<'a, T::Output, Sqlite, KU, X> {
        self.comparator::<X>().ternary_op(lower, upper, StandardOperator::Between)
    }

    #[inline(always)]
    pub fn not_between<L, U, KL, KU, X>(self, lower: L, upper: U) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            L: IntoOperand<'a, T::Output, Sqlite, KL, X>,
            U: IntoOperand<'a, T::Output, Sqlite, KU, X> {
        self.comparator::<X>().ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    #[inline(always)]
    pub fn in_list<I, V, K, X>(self, values: I) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        self.comparator::<X>().list_op(values, StandardOperator::InList)
    }

    #[inline(always)]
    pub fn not_in_list<I, V, K, X>(self, values: I) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        self.comparator::<X>().list_op(values, StandardOperator::NotInList)
    }

    #[inline(always)]
    pub fn in_select<V, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            V: IntoSelectStatement<'a, Sqlite, X> {
        self.comparator::<X>().select_op(v, StandardOperator::InSelect)
    }

    #[inline(always)]
    pub fn not_in_select<V, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: AstNode<'a, Sqlite, X>,
            V: IntoSelectStatement<'a, Sqlite, X> {
        self.comparator::<X>().select_op(v, StandardOperator::NotInSelect)
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn like<V, K, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: CanString<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, Static<String>, Sqlite, K, X> {
        Comparator::<'a, Static<String>, Sqlite>::new(self.0.into_ast_operand())
            .binary_op(v, StandardOperator::Like)
    }

    #[inline(always)]
    pub fn not_like<V, K, X>(self, v: V) -> Condition<'a, Sqlite, X>
        where
            T: CanString<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, Static<String>, Sqlite, K, X> {
        Comparator::<'a, Static<String>, Sqlite>::new(self.0.into_ast_operand())
            .binary_op(v, StandardOperator::NotLike)
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn add<V, K, X>(self, v: V) -> Node<Math<'a, Sqlite, T::Output, X>>
        where
            T: CanMath<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        Node(Math::new(MathOperator::Add, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn sub<V, K, X>(self, v: V) -> Node<Math<'a, Sqlite, T::Output, X>>
        where
            T: CanMath<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        Node(Math::new(MathOperator::Sub, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn mul<V, K, X>(self, v: V) -> Node<Math<'a, Sqlite, T::Output, X>>
        where
            T: CanMath<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        Node(Math::new(MathOperator::Mul, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn div<V, K, X>(self, v: V) -> Node<Math<'a, Sqlite, T::Output, X>>
        where
            T: CanMath<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        Node(Math::new(MathOperator::Div, self.0.into_ast_operand(), v.into_operand()))
    }
}

impl<'a, T> Node<T>
    where
        T: CanAlias {
    #[inline(always)]
    pub fn alias<A, U, X>(self, alias: A) -> Projection<'a, Sqlite, X>
        where
            Self: IntoProjection<'a, Sqlite, X>,
            A: IntoIdentifier<U> {
        self.into_projection().alias(alias)
    }
}

impl<'a, T> Node<T>
    where
        T: CanOrder {
    #[inline(always)]
    pub fn asc<X>(self) -> Ordering<'a, Sqlite, X>
        where
            Self: IntoOrdering<'a, Sqlite, X> {
        self.into_ordering().direction(Direction::Asc)
    }

    #[inline(always)]
    pub fn desc<X>(self) -> Ordering<'a, Sqlite, X>
        where
            Self: IntoOrdering<'a, Sqlite, X> {
        self.into_ordering().direction(Direction::Desc)
    }
}

impl<'a, T> Node<T>
    where
        T: CanAssignment + TypedNode {
    #[inline(always)]
    pub fn set<V, K, U, X>(self, v: V) -> Assignment<'a, Sqlite, X>
        where
            T: IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        Assignment::from(StandardAssignment::Value(self.0.into_identifier().meta, v.into_operand()))
    }

    #[inline(always)]
    pub fn excluded<U, X>(self) -> Assignment<'a, Sqlite, X>
        where
            T: IntoIdentifier<U> {
        Assignment::from(ExtAssignment::ExcludedColumn(self.0.into_identifier().meta))
    }

    #[inline(always)]
    pub fn increment<V, K, U, X>(self, v: V) -> Assignment<'a, Sqlite, X>
        where
            T: CanMath<X> + IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        Assignment::from(StandardAssignment::Increment(
            self.0.into_identifier().meta,
            v.into_operand(),
        ))
    }

    #[inline(always)]
    pub fn decrement<V, K, U, X>(self, v: V) -> Assignment<'a, Sqlite, X>
        where
            T: CanMath<X> + IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Sqlite, K, X> {
        Assignment::from(StandardAssignment::Decrement(
            self.0.into_identifier().meta,
            v.into_operand(),
        ))
    }
}

impl<T> Node<T>
    where
        T: CanDistinct + SetDistinct {
    #[inline(always)]
    pub fn distinct(self) -> Self {
        Node(self.0.set_distinct(Quantifier::Distinct))
    }
}

impl<'a, T> Node<T> {
    #[inline(always)]
    pub fn over<X>(self, window: Window<'a, Sqlite, X>) -> Self
        where
            T: CanWindow + SetWindow<'a, Sqlite, X> {
        Node(self.0.set_window(window.erase_x()))
    }

    #[inline(always)]
    pub fn filter<X>(self, cond: Condition<'a, Sqlite, X>) -> Self
        where
            T: CanFilter + SetFilter<'a, Sqlite, X> {
        Node(self.0.set_filter(cond.erase_x()))
    }

    #[inline(always)]
    pub fn order_by<A, M, X>(self, ordering: A) -> Self
        where
            T: CanOrderBy + SetOrderBy<'a, Sqlite, X>,
            A: IntoArg<Ordering<'a, Sqlite, X>, M> {
        Node(self.0.set_order_by(ordering.into_args_erased()))
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn json_extract<V, K, X>(self, path: V) -> JsonExtract<'a, X>
        where
            T: CanSqliteJson<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, Static<String>, Sqlite, K, X> {
        Node(InnerJsonExtract {
            base: self.0.into_ast_operand(),
            path: path.into_operand(),
            _marker: PhantomData::<(JsonExtractMarker, X)>,
        })
    }

    #[inline(always)]
    pub fn jsonb_extract<V, K, X>(self, path: V) -> JsonbExtract<'a, X>
        where
            T: CanSqliteJson<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, Static<String>, Sqlite, K, X> {
        Node(InnerJsonExtract {
            base: self.0.into_ast_operand(),
            path: path.into_operand(),
            _marker: PhantomData::<(JsonbExtractMarker, X)>,
        })
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn regexp<V, K, X>(self, pattern: V) -> Condition<'a, Sqlite, X>
        where
            T: CanSqliteString<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, Static<String>, Sqlite, K, X> {
        Condition::from(Operator::from(ExtOperator::Regexp(
            self.0.into_ast_operand(),
            pattern.into_operand(),
        )))
    }
}

impl<'a, T> Node<T> {
    #[inline(always)]
    pub fn matches<V, K, X>(self, pattern: V) -> Condition<'a, Sqlite, X>
        where
            T: CanSqliteFts<X> + AstNode<'a, Sqlite, X>,
            V: IntoOperand<'a, Static<String>, Sqlite, K, X> {
        Condition::from(Operator::from(ExtOperator::Match(
            self.0.into_ast_operand(),
            pattern.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn bm25<X>(self) -> Bm25<'a, X>
        where
            T: CanSqliteFts<X> {
        Node(InnerBm25 { table: self.0.fts_table(), weights: Vec::new(), _marker: PhantomData })
    }

    #[inline(always)]
    pub fn highlight<L, R, KL, KR, X>(self, col_index: i32, left: L, right: R) -> Highlight<'a, X>
        where
            T: CanSqliteFts<X>,
            L: IntoOperand<'a, Static<String>, Sqlite, KL, X>,
            R: IntoOperand<'a, Static<String>, Sqlite, KR, X> {
        Node(InnerHighlight {
            table: self.0.fts_table(),
            col_index,
            left: left.into_operand(),
            right: right.into_operand(),
            _marker: PhantomData,
        })
    }

    #[inline(always)]
    pub fn snippet<L, R, E, KL, KR, KE, X>(
        self, col_index: i32, left: L, right: R, ellipsis: E, max_tokens: i32,
    ) -> Snippet<'a, X>
        where
            T: CanSqliteFts<X>,
            L: IntoOperand<'a, Static<String>, Sqlite, KL, X>,
            R: IntoOperand<'a, Static<String>, Sqlite, KR, X>,
            E: IntoOperand<'a, Static<String>, Sqlite, KE, X> {
        Node(InnerSnippet {
            table: self.0.fts_table(),
            col_index,
            left: left.into_operand(),
            right: right.into_operand(),
            ellipsis: ellipsis.into_operand(),
            max_tokens,
            _marker: PhantomData,
        })
    }
}

impl<T, U> IntoIdentifier<U> for Node<T>
    where
        T: IntoIdentifier<U> {
    #[inline(always)]
    fn into_identifier(self) -> Identifier<U> {
        self.0.into_identifier()
    }
}

impl<'a, T, X> IntoProjection<'a, Sqlite, X> for Node<T>
    where
        T: CanAlias + AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoOrdering<'a, Sqlite, X> for Node<T>
    where
        T: CanOrder + AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn into_ordering(self) -> Ordering<'a, Sqlite, X> {
        Ordering::from_operand(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoGroupBy<'a, Sqlite, X> for Node<T>
    where
        T: CanGroup + AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn into_group_by(self) -> GroupBy<'a, Sqlite, X> {
        GroupBy::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoPartitionBy<'a, Sqlite, X> for Node<T>
    where
        T: CanGroup + AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn into_partition_by(self) -> PartitionBy<'a, Sqlite, X> {
        PartitionBy::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoConflict<'a, Sqlite, X> for Node<T>
    where
        T: CanConflict + AstNode<'a, Sqlite, X> {
    #[inline(always)]
    fn into_conflict(self) -> Conflict<'a, Sqlite, X> {
        Conflict::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoArg<Projection<'a, Sqlite, X>, ArgAst> for Node<T>
    where
        Self: IntoProjection<'a, Sqlite, X> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}

impl<'a, T, X> IntoArg<Ordering<'a, Sqlite, X>, ArgAst> for Node<T>
    where
        Self: IntoOrdering<'a, Sqlite, X> {
    #[inline(always)]
    fn into_arg(self) -> Ordering<'a, Sqlite, X> {
        self.into_ordering()
    }
}

impl<'a, T, X> IntoArg<GroupBy<'a, Sqlite, X>, ArgAst> for Node<T>
    where
        Self: IntoGroupBy<'a, Sqlite, X> {
    #[inline(always)]
    fn into_arg(self) -> GroupBy<'a, Sqlite, X> {
        self.into_group_by()
    }
}

impl<'a, T, X> IntoArg<Conflict<'a, Sqlite, X>, ArgAst> for Node<T>
    where
        Self: IntoConflict<'a, Sqlite, X> {
    #[inline(always)]
    fn into_arg(self) -> Conflict<'a, Sqlite, X> {
        self.into_conflict()
    }
}

impl<'a, T, X> IntoArg<PartitionBy<'a, Sqlite, X>, ArgAst> for Node<T>
    where
        Self: IntoPartitionBy<'a, Sqlite, X> {
    #[inline(always)]
    fn into_arg(self) -> PartitionBy<'a, Sqlite, X> {
        self.into_partition_by()
    }
}
