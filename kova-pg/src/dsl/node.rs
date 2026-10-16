#![allow(clippy::wrong_self_convention)]

use crate::dsl::jsonb_chain::JsonbChain;
use crate::dsl::jsonb_text::JsonbText;
use crate::dsl::operator::JsonOp;
use crate::dsl::{ExtAssignment, ExtFunction, ExtOperator};
use crate::types::Pg;
use kova_core::dsl::func::{Quantifier, SetDistinct, SetFilter, SetOrderBy, SetWindow, Window};
use kova_core::dsl::{
    Assignment, AstNode, CanAlias, CanAssignment, CanCompare, CanDistinct, CanFilter, CanGroup,
    CanMath, CanOrder, CanOrderBy, CanString, CanWindow, Close, Comparator, Compatible, Condition,
    Direction, Dynamic, EraseX, GroupBy, Identifier, IntoArg, IntoArgs, IntoGroupBy,
    IntoIdentifier, IntoOperand, IntoOrdering, IntoProjection, Math, MathOperator, Operand,
    Operator, Ordering, Projection, SingleArgMarker, StandardAssignment, StandardOperator, Static,
    TypedNode,
};
use kova_core::type_id;
use kova_core::types::SetTag;

#[derive(Copy, Clone)]
pub struct Node<T>(pub T);

impl<T: TypedNode> TypedNode for Node<T> {
    type Output = T::Output;
}

impl<'a, T, X> AstNode<'a, Pg, X> for Node<T>
    where
        T: AstNode<'a, Pg, X> {
    #[inline(always)]
    fn into_ast_operand(self) -> Operand<'a, Pg> {
        self.0.into_ast_operand()
    }
}

impl<T> SingleArgMarker for Node<T> {}

impl<'a, T, Target, X> IntoOperand<'a, Target, Pg, Close, X> for Node<T>
    where
        T: AstNode<'a, Pg, X>,
        T::Output: Compatible<Target, X> {
    #[inline(always)]
    fn into_operand(self) -> Operand<'a, Pg> {
        self.0.into_ast_operand()
    }
}

impl<T> Node<T> {
    #[inline(always)]
    pub fn tag<Marker: 'static>(mut self) -> Self
        where
            T: SetTag {
        self.0.set_tag(type_id::<Marker>());
        self
    }

    #[inline(always)]
    fn comparator<'a, X>(self) -> Comparator<'a, T::Output, Pg>
        where
            T: AstNode<'a, Pg, X> {
        Comparator::new(self.0.into_ast_operand())
    }
}

impl<'a, T> Node<T>
    where
        T: CanCompare + TypedNode {
    #[inline(always)]
    pub fn eq<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Eq)
    }

    #[inline(always)]
    pub fn ne<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Ne)
    }

    #[inline(always)]
    pub fn gt<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Gt)
    }

    #[inline(always)]
    pub fn ge<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Ge)
    }

    #[inline(always)]
    pub fn lt<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Lt)
    }

    #[inline(always)]
    pub fn le<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Le)
    }

    #[inline(always)]
    pub fn is_null<X>(self) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X> {
        self.comparator::<X>().unary_op(StandardOperator::IsNull)
    }

    #[inline(always)]
    pub fn is_not_null<X>(self) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X> {
        self.comparator::<X>().unary_op(StandardOperator::IsNotNull)
    }

    #[inline(always)]
    pub fn between<L, U, KL, KU, X>(self, lower: L, upper: U) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            L: IntoOperand<'a, T::Output, Pg, KL, X>,
            U: IntoOperand<'a, T::Output, Pg, KU, X> {
        self.comparator::<X>().ternary_op(lower, upper, StandardOperator::Between)
    }

    #[inline(always)]
    pub fn not_between<L, U, KL, KU, X>(self, lower: L, upper: U) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            L: IntoOperand<'a, T::Output, Pg, KL, X>,
            U: IntoOperand<'a, T::Output, Pg, KU, X> {
        self.comparator::<X>().ternary_op(lower, upper, StandardOperator::NotBetween)
    }

    #[inline(always)]
    pub fn in_list<I, V, K, X>(self, values: I) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().list_op(values, StandardOperator::InList)
    }

    #[inline(always)]
    pub fn not_in_list<I, V, K, X>(self, values: I) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().list_op(values, StandardOperator::NotInList)
    }

    #[inline(always)]
    pub fn in_select<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Dynamic, Pg, K, X> {
        Comparator::<'a, Dynamic, Pg>::new(self.0.into_ast_operand())
            .binary_op(v, StandardOperator::InSelect)
    }

    #[inline(always)]
    pub fn not_in_select<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Dynamic, Pg, K, X> {
        Comparator::<'a, Dynamic, Pg>::new(self.0.into_ast_operand())
            .binary_op(v, StandardOperator::NotInSelect)
    }
}

impl<'a, T> Node<T>
    where
        T: CanString + TypedNode {
    #[inline(always)]
    pub fn like<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::Like)
    }

    #[inline(always)]
    pub fn not_like<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        self.comparator::<X>().binary_op(v, StandardOperator::NotLike)
    }
}

impl<'a, T> Node<T>
    where
        T: CanMath + TypedNode {
    #[inline(always)]
    pub fn add<V, K, X>(self, v: V) -> Node<Math<'a, Pg, T::Output, X>>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Node(Math::new(MathOperator::Add, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn sub<V, K, X>(self, v: V) -> Node<Math<'a, Pg, T::Output, X>>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Node(Math::new(MathOperator::Sub, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn mul<V, K, X>(self, v: V) -> Node<Math<'a, Pg, T::Output, X>>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Node(Math::new(MathOperator::Mul, self.0.into_ast_operand(), v.into_operand()))
    }

    #[inline(always)]
    pub fn div<V, K, X>(self, v: V) -> Node<Math<'a, Pg, T::Output, X>>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Node(Math::new(MathOperator::Div, self.0.into_ast_operand(), v.into_operand()))
    }
}


impl<'a, T> Node<T>
    where
        T: CanAlias {
    #[inline(always)]
    pub fn alias<A, U, X>(self, alias: A) -> Projection<'a, Pg, X>
        where
            Self: IntoProjection<'a, Pg, X>,
            A: IntoIdentifier<U> {
        self.into_projection().alias(alias)
    }
}

impl<'a, T> Node<T>
    where
        T: CanOrder {
    #[inline(always)]
    pub fn asc<X>(self) -> Ordering<'a, Pg, X>
        where
            Self: IntoOrdering<'a, Pg, X> {
        self.into_ordering().direction(Direction::Asc)
    }

    #[inline(always)]
    pub fn desc<X>(self) -> Ordering<'a, Pg, X>
        where
            Self: IntoOrdering<'a, Pg, X> {
        self.into_ordering().direction(Direction::Desc)
    }
}

impl<'a, T> Node<T>
    where
        T: CanAssignment + TypedNode {
    #[inline(always)]
    pub fn set<V, K, U, X>(self, v: V) -> Assignment<'a, Pg, X>
        where
            T: IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Assignment::from(StandardAssignment::Value(self.0.into_identifier().meta, v.into_operand()))
    }

    #[inline(always)]
    pub fn excluded<U, X>(self) -> Assignment<'a, Pg, X>
        where
            T: IntoIdentifier<U> {
        Assignment::from(ExtAssignment::ExcludedColumn(self.0.into_identifier().meta))
    }

    #[inline(always)]
    pub fn increment<V, K, U, X>(self, v: V) -> Assignment<'a, Pg, X>
        where
            T: IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Assignment::from(StandardAssignment::Increment(
            self.0.into_identifier().meta,
            v.into_operand(),
        ))
    }

    #[inline(always)]
    pub fn decrement<V, K, U, X>(self, v: V) -> Assignment<'a, Pg, X>
        where
            T: IntoIdentifier<U>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
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
    pub fn over<X>(self, window: Window<'a, Pg, X>) -> Self
        where
            T: CanWindow + SetWindow<'a, Pg, X> {
        Node(self.0.set_window(window.erase_x()))
    }

    #[inline(always)]
    pub fn filter<X>(self, cond: Condition<'a, Pg, X>) -> Self
        where
            T: CanFilter + SetFilter<'a, Pg, X> {
        Node(self.0.set_filter(cond.erase_x()))
    }

    #[inline(always)]
    pub fn order_by<A, X>(self, ordering: A) -> Self
        where
            T: CanOrderBy + SetOrderBy<'a, Pg, X>,
            A: IntoArgs<Ordering<'a, Pg, X>> {
        Node(self.0.set_order_by(ordering.into_mapped_args(|o| o.erase_x())))
    }
}

pub trait CanPgString {}
pub trait CanPgArray {}
pub trait CanPgJsonb {}

impl<'a, T> Node<T>
    where
        T: CanPgString + TypedNode {
    #[inline(always)]
    pub fn ilike<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::ILike(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn not_ilike<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::NotILike(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn matches<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::Matches(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn not_matches<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::NotMatches(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn imatches<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::IMatches(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn not_imatches<V, K, X>(self, v: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::NotIMatches(
            self.0.into_ast_operand(),
            v.into_operand(),
        )))
    }
}

impl<'a, T> Node<T>
    where
        T: CanPgArray + TypedNode {
    #[inline(always)]
    pub fn array_contains<V, K, X>(self, val: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::ArrayContains(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn array_contained_by<V, K, X>(self, val: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::ArrayContainedBy(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn array_overlap<V, K, X>(self, val: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, T::Output, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::ArrayOverlap(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }
}

impl<'a, T> Node<T>
    where
        T: CanPgJsonb + TypedNode {
    #[inline(always)]
    pub fn json_path<V, K, X>(self, key: V) -> JsonbChain<'a, T, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        JsonbChain(self.0.into_ast_operand(), vec![JsonOp::Path(key.into_operand())])
    }

    #[inline(always)]
    pub fn json_path_array<I, V, K, X>(self, path: I) -> JsonbChain<'a, T, X>
        where
            T: AstNode<'a, Pg, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        JsonbChain(self.0.into_ast_operand(), vec![JsonOp::PathArray(ExtFunction::from_iter(path))])
    }

    #[inline(always)]
    pub fn json_concat<V, K, X>(self, val: V) -> JsonbChain<'a, T, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Dynamic, Pg, K, X> {
        JsonbChain(self.0.into_ast_operand(), vec![JsonOp::Concat(val.into_operand())])
    }

    #[inline(always)]
    pub fn json_delete<V, K, X>(self, key: V) -> JsonbChain<'a, T, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        JsonbChain(self.0.into_ast_operand(), vec![JsonOp::Delete(key.into_operand())])
    }

    #[inline(always)]
    pub fn json_delete_path<I, V, K, X>(self, path: I) -> JsonbChain<'a, T, X>
        where
            T: AstNode<'a, Pg, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        JsonbChain(
            self.0.into_ast_operand(),
            vec![JsonOp::DeletePath(ExtFunction::from_iter(path))],
        )
    }

    #[inline(always)]
    pub fn json_path_text<V, K, X>(self, key: V) -> JsonbText<'a, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        JsonbText(self.0.into_ast_operand(), vec![JsonOp::PathText(key.into_operand())])
    }

    #[inline(always)]
    pub fn json_path_text_array<I, V, K, X>(self, path: I) -> JsonbText<'a, X>
        where
            T: AstNode<'a, Pg, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        JsonbText(
            self.0.into_ast_operand(),
            vec![JsonOp::PathTextArray(ExtFunction::from_iter(path))],
        )
    }

    #[inline(always)]
    pub fn json_contains<V, K, X>(self, val: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Dynamic, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonContains(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn json_contained_by<V, K, X>(self, val: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Dynamic, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonContainedBy(
            self.0.into_ast_operand(),
            val.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn json_has_key<V, K, X>(self, key: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasKey(
            self.0.into_ast_operand(),
            key.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn json_has_any_keys<I, V, K, X>(self, keys: I) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasAnyKeys(
            self.0.into_ast_operand(),
            ExtFunction::from_iter(keys),
        )))
    }

    #[inline(always)]
    pub fn json_has_all_keys<I, V, K, X>(self, keys: I) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            I: IntoIterator<Item = V>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonHasAllKeys(
            self.0.into_ast_operand(),
            ExtFunction::from_iter(keys),
        )))
    }

    #[inline(always)]
    pub fn json_path_exists<V, K, X>(self, path: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
        Condition::from(Operator::from(ExtOperator::JsonPathExists(
            self.0.into_ast_operand(),
            path.into_operand(),
        )))
    }

    #[inline(always)]
    pub fn json_path_predicate<V, K, X>(self, path: V) -> Condition<'a, Pg, X>
        where
            T: AstNode<'a, Pg, X>,
            V: IntoOperand<'a, Static<String>, Pg, K, X> {
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

impl<'a, T, X> IntoProjection<'a, Pg, X> for Node<T>
    where
        T: CanAlias + AstNode<'a, Pg, X> {
    #[inline(always)]
    fn into_projection(self) -> Projection<'a, Pg, X> {
        Projection::new(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoOrdering<'a, Pg, X> for Node<T>
    where
        T: CanOrder + AstNode<'a, Pg, X> {
    #[inline(always)]
    fn into_ordering(self) -> Ordering<'a, Pg, X> {
        Ordering::from_operand(self.0.into_ast_operand())
    }
}

impl<'a, T, X> IntoGroupBy<'a, Pg, X> for Node<T>
    where
        T: CanGroup + AstNode<'a, Pg, X> {
    #[inline(always)]
    fn into_group_by(self) -> GroupBy<'a, Pg, X> {
        GroupBy::new(self.0.into_ast_operand())
    }
}

impl<'a, T> IntoArg<Operand<'a, Pg>> for Node<T>
    where
        T: AstNode<'a, Pg, ()> {
    #[inline(always)]
    fn into_arg(self) -> Operand<'a, Pg> {
        self.0.into_ast_operand()
    }
}

impl<'a, T, X> IntoArg<Projection<'a, Pg, X>> for Node<T>
    where
        Self: IntoProjection<'a, Pg, X> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, Pg, X> {
        self.into_projection()
    }
}

impl<'a, T, X> IntoArg<Ordering<'a, Pg, X>> for Node<T>
    where
        Self: IntoOrdering<'a, Pg, X> {
    #[inline(always)]
    fn into_arg(self) -> Ordering<'a, Pg, X> {
        self.into_ordering()
    }
}

impl<'a, T, X> IntoArg<GroupBy<'a, Pg, X>> for Node<T>
    where
        Self: IntoGroupBy<'a, Pg, X> {
    #[inline(always)]
    fn into_arg(self) -> GroupBy<'a, Pg, X> {
        self.into_group_by()
    }
}
