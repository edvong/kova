#![allow(clippy::wrong_self_convention)]

use crate::dsl::can::{CanPgArray, CanPgJsonb, CanPgString};
use crate::dsl::function::ExtFunction;
use crate::dsl::jsonb_chain::JsonbChain;
use crate::dsl::jsonb_text::JsonbText;
use crate::dsl::operator::JsonOp;
use crate::dsl::{ExtAssignment, ExtOperator};
use crate::types::Postgres;
use kova_kernel::dsl::func::{
    IntoPartitionBy, PartitionBy, Quantifier, SetDistinct, SetFilter, SetOrderBy, SetWindow, Window,
};
use kova_kernel::dsl::statement::IntoSelectStatement;
use kova_kernel::dsl::{
    ArgAst, Assignment, AstNode, CanAlias, CanAssignment, CanCompare, CanConflict, CanDistinct,
    CanFilter, CanGroup, CanMath, CanOrder, CanOrderBy, CanString, CanWindow, Close, Comparator,
    Compatible, Condition, Conflict, Direction, Dynamic, EraseX, GroupBy, Identifier, IntoArg,
    IntoConflict, IntoGroupBy, IntoIdentifier, IntoOperand, IntoOrdering, IntoProjection, Math,
    MathOperator, Operand, Operator, Ordering, Projection, StandardAssignment, StandardOperator,
    Static, TypedNode,
};
use kova_kernel::types::SetTag;
use std::any::TypeId;

#[derive(Copy, Clone)]
pub struct Node<T>(pub T);

impl<T: CanGroup> CanGroup for Node<T> {}
impl<T: CanConflict> CanConflict for Node<T> {}
impl<T: CanMath<X>, X> CanMath<X> for Node<T> {}
impl<T: CanString<X>, X> CanString<X> for Node<T> {}
impl<T: CanPgArray<X>, X> CanPgArray<X> for Node<T> {}
impl<T: CanPgJsonb<X>, X> CanPgJsonb<X> for Node<T> {}

impl<T: TypedNode> TypedNode for Node<T> {
    type Output = T::Output;
}

impl<'a, T, X> AstNode<'a, Postgres, X> for Node<T>
    where
        T: AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Postgres> {
        self.0.into_ast_operand()
    }
}

#[diagnostic::do_not_recommend]
impl<'a, T, Target, X> IntoOperand<'a, Target, Postgres, Close, X> for Node<T>
    where
        T: AstNode<'a, Postgres, X>,
        T::Output: Compatible<Target, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Postgres> {
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
    fn comparator<'a, X>(self) -> Comparator<'a, T::Output, Postgres>
        where
            T: AstNode<'a, Postgres, X> {
        Comparator::new(self.0.into_ast_operand())
    }
}

impl<'a, T> Node<T>
    where
        T: CanCompare + TypedNode {
    #[inline(always)]
    pub fn eq<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Eq)
    }

    #[inline(always)]
    pub fn ne<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Ne)
    }

    #[inline(always)]
    pub fn gt<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Gt)
    }

    #[inline(always)]
    pub fn ge<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Ge)
    }

    #[inline(always)]
    pub fn lt<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Lt)
    }

    #[inline(always)]
    pub fn le<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Le)
    }

    #[inline(always)]
    pub fn is_null<X>(self) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X> {
        self.comparator::<X>().unary_op(StandardOperator::IsNull)
    }

    #[inline(always)]
    pub fn is_not_null<X>(self) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X> {
        self.comparator::<X>().unary_op(StandardOperator::IsNotNull)
    }

    #[inline(always)]
    pub fn between<L, U, KL, KU, X>(self, lower: L, upper: U) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            L: IntoOperand<'a, T::Output, Postgres, KL, X>,
            U: IntoOperand<'a, T::Output, Postgres, KU, X> {
        self.comparator::<X>().ternary_op(lower, upper, StandardOperator::Between)
    }

    #[inline(always)]
    pub fn not_between<L, U, KL, KU, X>(self, lower: L, upper: U) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            L: IntoOperand<'a, T::Output, Postgres, KL, X>,
            U: IntoOperand<'a, T::Output, Postgres, KU, X> {
        self.comparator::<X>().ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    #[inline(always)]
    pub fn in_list<I, V, K, X>(self, values: I) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        self.comparator::<X>().list_op(values, StandardOperator::InList)
    }

    #[inline(always)]
    pub fn not_in_list<I, V, K, X>(self, values: I) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        self.comparator::<X>().list_op(values, StandardOperator::NotInList)
    }

    #[inline(always)]
    pub fn in_select<V, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            V: IntoSelectStatement<'a, Postgres, X> {
        self.comparator::<X>().select_op(v, StandardOperator::InSelect)
    }

    #[inline(always)]
    pub fn not_in_select<V, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: AstNode<'a, Postgres, X>,
            V: IntoSelectStatement<'a, Postgres, X> {
        self.comparator::<X>().select_op(v, StandardOperator::NotInSelect)
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn like<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: CanString<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Comparator::<'a, Static<String>, Postgres>::new(self.0.into_ast_operand())
            .binary_op(v, StandardOperator::Like)
    }

    #[inline(always)]
    pub fn not_like<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: CanString<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Comparator::<'a, Static<String>, Postgres>::new(self.0.into_ast_operand())
            .binary_op(v, StandardOperator::NotLike)
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn add<V, K, X>(self, v: V) -> Node<Math<'a, Postgres, T::Output, X>>
        where
            T: CanMath<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Node(Math::new(MathOperator::Add, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn sub<V, K, X>(self, v: V) -> Node<Math<'a, Postgres, T::Output, X>>
        where
            T: CanMath<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Node(Math::new(MathOperator::Sub, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn mul<V, K, X>(self, v: V) -> Node<Math<'a, Postgres, T::Output, X>>
        where
            T: CanMath<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Node(Math::new(MathOperator::Mul, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn div<V, K, X>(self, v: V) -> Node<Math<'a, Postgres, T::Output, X>>
        where
            T: CanMath<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Node(Math::new(MathOperator::Div, self.0.into_ast_operand(), v.into_operand()))
    }
}

impl<'a, T> Node<T>
    where
        T: CanAlias {
    #[inline(always)]
    pub fn alias<A, U, X>(self, alias: A) -> Projection<'a, Postgres, X>
        where
            Self: IntoProjection<'a, Postgres, X>,
            A: IntoIdentifier<U> {
        self.into_projection().alias(alias)
    }
}

impl<'a, T> Node<T>
    where
        T: CanOrder {
    #[inline(always)]
    pub fn asc<X>(self) -> Ordering<'a, Postgres, X>
        where
            Self: IntoOrdering<'a, Postgres, X> {
        self.into_ordering().direction(Direction::Asc)
    }

    #[inline(always)]
    pub fn desc<X>(self) -> Ordering<'a, Postgres, X>
        where
            Self: IntoOrdering<'a, Postgres, X> {
        self.into_ordering().direction(Direction::Desc)
    }
}

impl<'a, T> Node<T>
    where
        T: CanAssignment + TypedNode {
    #[inline(always)]
    pub fn set<V, K, U, X>(self, v: V) -> Assignment<'a, Postgres, X>
        where
            T: IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Assignment::from(StandardAssignment::Value(self.0.into_identifier().meta, v.into_operand()))
    }

    #[inline(always)]
    pub fn excluded<U, X>(self) -> Assignment<'a, Postgres, X>
        where
            T: IntoIdentifier<U> {
        Assignment::from(ExtAssignment::ExcludedColumn(self.0.into_identifier().meta))
    }

    #[inline(always)]
    pub fn increment<V, K, U, X>(self, v: V) -> Assignment<'a, Postgres, X>
        where
            T: CanMath<X> + IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Assignment::from(StandardAssignment::Increment(
            self.0.into_identifier().meta,
            v.into_operand(),
        ))
    }

    #[inline(always)]
    pub fn decrement<V, K, U, X>(self, v: V) -> Assignment<'a, Postgres, X>
        where
            T: CanMath<X> + IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
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
    pub fn over<X>(self, window: Window<'a, Postgres, X>) -> Self
        where
            T: CanWindow + SetWindow<'a, Postgres, X> {
        Node(self.0.set_window(window.erase_x()))
    }

    #[inline(always)]
    pub fn filter<X>(self, cond: Condition<'a, Postgres, X>) -> Self
        where
            T: CanFilter + SetFilter<'a, Postgres, X> {
        Node(self.0.set_filter(cond.erase_x()))
    }

    #[inline(always)]
    pub fn order_by<A, M, X>(self, ordering: A) -> Self
        where
            T: CanOrderBy + SetOrderBy<'a, Postgres, X>,
            A: IntoArg<Ordering<'a, Postgres, X>, M> {
        Node(self.0.set_order_by(ordering.into_args_erased()))
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn ilike<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgString<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::ILike(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn not_ilike<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgString<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::NotILike(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn matches<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgString<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::Matches(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn not_matches<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgString<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::NotMatches(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn imatches<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgString<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::IMatches(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn not_imatches<V, K, X>(self, v: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgString<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::NotIMatches(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn array_contains<V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgArray<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::ArrayContains(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn array_contained_by<V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgArray<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::ArrayContainedBy(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn array_overlap<V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgArray<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, T::Output, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::ArrayOverlap(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }
}

impl<'a, T: TypedNode> Node<T> {
    #[inline(always)]
    pub fn json_path<V, K, X>(self, key: V) -> JsonbChain<'a, T, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        JsonbChain(self.0.into_ast_operand(), JsonOp::Path(key.into_operand()))
    }

    #[inline(always)]
    pub fn json_path_array<I, V, K, X>(self, path: I) -> JsonbChain<'a, T, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        JsonbChain(self.0.into_ast_operand(), JsonOp::PathArray(ExtFunction::from_iter(path)))
    }

    #[inline(always)]
    pub fn json_concat<V, K, X>(self, val: V) -> JsonbChain<'a, T, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Dynamic, Postgres, K, X> {
        JsonbChain(self.0.into_ast_operand(), JsonOp::Concat(val.into_operand()))
    }

    #[inline(always)]
    pub fn json_delete<V, K, X>(self, key: V) -> JsonbChain<'a, T, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        JsonbChain(self.0.into_ast_operand(), JsonOp::Delete(key.into_operand()))
    }

    #[inline(always)]
    pub fn json_delete_path<I, V, K, X>(self, path: I) -> JsonbChain<'a, T, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        JsonbChain(self.0.into_ast_operand(), JsonOp::DeletePath(ExtFunction::from_iter(path)))
    }

    #[inline(always)]
    pub fn json_path_text<V, K, X>(self, key: V) -> JsonbText<'a, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        JsonbText(self.0.into_ast_operand(), JsonOp::PathText(key.into_operand()))
    }

    #[inline(always)]
    pub fn json_path_text_array<I, V, K, X>(self, path: I) -> JsonbText<'a, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        JsonbText(self.0.into_ast_operand(), JsonOp::PathTextArray(ExtFunction::from_iter(path)))
    }

    #[inline(always)]
    pub fn json_contains<V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Dynamic, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonContains(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn json_contained_by<V, K, X>(self, val: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Dynamic, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonContainedBy(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn json_has_key<V, K, X>(self, key: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasKey(
            self.0.into_ast_operand(),
            key.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn json_has_any_keys<I, V, K, X>(self, keys: I) -> Condition<'a, Postgres, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasAnyKeys(
            self.0.into_ast_operand(),
            ExtFunction::from_iter(keys),
        )))
    }

    #[inline(always)]
    pub fn json_has_all_keys<I, V, K, X>(self, keys: I) -> Condition<'a, Postgres, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasAllKeys(
            self.0.into_ast_operand(),
            ExtFunction::from_iter(keys),
        )))
    }

    #[inline(always)]
    pub fn json_path_exists<V, K, X>(self, path: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonPathExists(
            self.0.into_ast_operand(),
            path.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn json_path_predicate<V, K, X>(self, path: V) -> Condition<'a, Postgres, X>
        where
            T: CanPgJsonb<X> + AstNode<'a, Postgres, X>,
            V: IntoOperand<'a, Static<String>, Postgres, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonPathPredicate(
            self.0.into_ast_operand(),
            path.into_operand(),
        )))
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

impl<'a, T, X> IntoProjection<'a, Postgres, X> for Node<T>
    where
        T: CanAlias + AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn into_projection(self) -> Projection<'a, Postgres, X> {
        Projection::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoOrdering<'a, Postgres, X> for Node<T>
    where
        T: CanOrder + AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn into_ordering(self) -> Ordering<'a, Postgres, X> {
        Ordering::from_operand(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoGroupBy<'a, Postgres, X> for Node<T>
    where
        T: CanGroup + AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn into_group_by(self) -> GroupBy<'a, Postgres, X> {
        GroupBy::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoPartitionBy<'a, Postgres, X> for Node<T>
    where
        T: CanGroup + AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn into_partition_by(self) -> PartitionBy<'a, Postgres, X> {
        PartitionBy::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoConflict<'a, Postgres, X> for Node<T>
    where
        T: CanConflict + AstNode<'a, Postgres, X> {
    #[inline(always)]
    fn into_conflict(self) -> Conflict<'a, Postgres, X> {
        Conflict::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoArg<Projection<'a, Postgres, X>, ArgAst> for Node<T>
    where
        Self: IntoProjection<'a, Postgres, X> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, Postgres, X> {
        self.into_projection()
    }
}

impl<'a, T, X> IntoArg<Ordering<'a, Postgres, X>, ArgAst> for Node<T>
    where
        Self: IntoOrdering<'a, Postgres, X> {
    #[inline(always)]
    fn into_arg(self) -> Ordering<'a, Postgres, X> {
        self.into_ordering()
    }
}

impl<'a, T, X> IntoArg<GroupBy<'a, Postgres, X>, ArgAst> for Node<T>
    where
        Self: IntoGroupBy<'a, Postgres, X> {
    #[inline(always)]
    fn into_arg(self) -> GroupBy<'a, Postgres, X> {
        self.into_group_by()
    }
}

impl<'a, T, X> IntoArg<Conflict<'a, Postgres, X>, ArgAst> for Node<T>
    where
        Self: IntoConflict<'a, Postgres, X> {
    #[inline(always)]
    fn into_arg(self) -> Conflict<'a, Postgres, X> {
        self.into_conflict()
    }
}

impl<'a, T, X> IntoArg<PartitionBy<'a, Postgres, X>, ArgAst> for Node<T>
    where
        Self: IntoPartitionBy<'a, Postgres, X> {
    #[inline(always)]
    fn into_arg(self) -> PartitionBy<'a, Postgres, X> {
        self.into_partition_by()
    }
}
