use crate::dsl::func::{IntoSrfFunction, JsonSrfArgs};
use crate::query::exec::fetch_rows;
use crate::types::Sqlite;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::{IntoSelectStatement, SelectStatement};
use kova_core::dsl::{
    ArgAst, Close, Condition, Dynamic, EraseX, FromItem, GroupBy, ValueSource, IntoArg,
    IntoIdentifier, IntoInsertSource, IntoJoinConstraint, IntoOperand, IntoProjection, JoinType,
    Operand, Ordering, Projection, SetOperator, TypedNode,
};
use kova_core::types::{
    Cte, ExecutableSync, FromColumn, FromRow, Model, Relation, Select as CoreSelect,
};
use std::iter::once;
use std::marker::PhantomData;

pub struct OuterQuery<'c, E>(PhantomData<&'c E>);
pub struct InnerQuery;

pub trait Dispatch {
    type Executor;
}
impl<'c, E> Dispatch for OuterQuery<'c, E> {
    type Executor = &'c Executor<E>;
}
impl Dispatch for InnerQuery {
    type Executor = ();
}

pub struct Normal;
pub struct JustAddFrom;
pub struct TaggedFrom;
pub struct JustAddSrf;
pub struct JustAddJoin;

pub struct Select<'a, T, D: Dispatch, TAG, X> {
    pub(crate) executor: D::Executor,
    pub(crate) statement: SelectStatement<'a, Sqlite>,
    _marker: PhantomData<(T, TAG, X)>,
}

impl<'c, 'a, T, E, X> CoreSelect<'c, 'a, Sqlite, T, E, X>
    for Select<'a, T, OuterQuery<'c, E>, JustAddFrom, X>
    where
        T: Model<DB = Sqlite> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: SelectStatement::new(T::TABLE_NAME), _marker: PhantomData }
    }
}

// Initial state: either .tag() or append .srf() to the main table.
impl<'a, T, D: Dispatch, X> Select<'a, T, D, JustAddFrom, X>
    where
        T: Model<DB = Sqlite> {
    pub fn tag<Marker: 'static>(mut self) -> Select<'a, T, D, TaggedFrom, X> {
        self.statement.tag_latest_from::<Marker>();
        self.into_state()
    }

    pub fn srf<A: IntoArg<FromItem<'a, Sqlite, X>, M>, M>(
        mut self, args: A,
    ) -> Select<'a, T, D, JustAddSrf, X> {
        self.statement.srf(args.into_args_erased());
        self.into_state()
    }
}

// Main table tagged status: still allows appending once .srf()
impl<'a, T, D: Dispatch, X> Select<'a, T, D, TaggedFrom, X>
    where
        T: Model<DB = Sqlite> {
    pub fn srf<A: IntoArg<FromItem<'a, Sqlite, X>, M>, M>(
        mut self, args: A,
    ) -> Select<'a, T, D, JustAddSrf, X> {
        self.statement.srf(args.into_args_erased());
        self.into_state()
    }
}

// Just appended SRF status: .tag() is allowed for this SRF, but the .srf() method no longer exists.
impl<'a, T, D: Dispatch, X> Select<'a, T, D, JustAddSrf, X>
    where
        T: Model<DB = Sqlite> {
    pub fn tag<Marker: 'static>(mut self) -> Select<'a, T, D, Normal, X> {
        self.statement.tag_latest_from::<Marker>();
        self.into_state()
    }
}

impl<'c, 'a, T, D: Dispatch, X> Select<'a, T, D, JustAddJoin, X>
    where
        T: Model<DB = Sqlite> {
    pub fn tag<Marker: 'static>(mut self) -> Select<'a, T, D, Normal, X> {
        self.statement.tag_latest_join::<Marker>();
        self.into_state()
    }
}

impl<'a, T, D: Dispatch, TAG, X> Select<'a, T, D, TAG, X>
    where
        T: Model<DB = Sqlite> {
    fn into_state<State>(self) -> Select<'a, T, D, State, X> {
        Select { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }

    pub fn with<C: Cte>(
        mut self, query: impl IntoSelectStatement<'a, Sqlite, X>,
    ) -> Select<'a, T, D, Normal, X> {
        self.statement.with::<C>(query.into_select_statement());
        self.into_state()
    }

    pub fn distinct(mut self) -> Select<'a, T, D, Normal, X> {
        self.statement.distinct();
        self.into_state()
    }

    pub fn columns<A, M>(mut self, cols: A) -> Select<'a, T, D, Normal, X>
        where
            A: IntoArg<Projection<'a, Sqlite, X>, M> {
        self.statement.columns(cols.into_args_erased());
        self.into_state()
    }

    pub fn inner_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, Sqlite, X>,
    ) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.join::<R, _, X>(JoinType::Inner, constraint);
        self.into_state()
    }

    pub fn left_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, Sqlite, X>,
    ) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.join::<R, _, X>(JoinType::Left, constraint);
        self.into_state()
    }

    pub fn right_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, Sqlite, X>,
    ) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.join::<R, _, X>(JoinType::Right, constraint);
        self.into_state()
    }

    pub fn full_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, Sqlite, X>,
    ) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.join::<R, _, X>(JoinType::Full, constraint);
        self.into_state()
    }

    pub fn cross_join<R: Relation>(mut self) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.cross_join::<R>();
        self.into_state()
    }

    pub fn filter(mut self, cond: Condition<'a, Sqlite, X>) -> Select<'a, T, D, Normal, X> {
        self.statement.r#where(cond.erase_x());
        self.into_state()
    }

    pub fn group_by<A, M>(mut self, cols: A) -> Select<'a, T, D, Normal, X>
        where
            A: IntoArg<GroupBy<'a, Sqlite, X>, M> {
        self.statement.group_by(cols.into_args_erased());
        self.into_state()
    }

    pub fn having(mut self, cond: Condition<'a, Sqlite, X>) -> Select<'a, T, D, Normal, X> {
        self.statement.having(cond.erase_x());
        self.into_state()
    }

    pub fn order_by<A, M>(mut self, cols: A) -> Select<'a, T, D, Normal, X>
        where
            A: IntoArg<Ordering<'a, Sqlite, X>, M> {
        self.statement.order_by(cols.into_args_erased());
        self.into_state()
    }

    pub fn limit(mut self, limit: usize) -> Select<'a, T, D, Normal, X> {
        self.statement.limit(limit);
        self.into_state()
    }

    pub fn offset(mut self, offset: usize) -> Select<'a, T, D, Normal, X> {
        self.statement.offset(offset);
        self.into_state()
    }

    pub fn union(
        mut self, other: impl IntoSelectStatement<'a, Sqlite, X>,
    ) -> Select<'a, T, D, Normal, X> {
        self.statement.combine(SetOperator::Union, other.into_select_statement());
        self.into_state()
    }

    pub fn union_all(
        mut self, other: impl IntoSelectStatement<'a, Sqlite, X>,
    ) -> Select<'a, T, D, Normal, X> {
        self.statement.combine(SetOperator::UnionAll, other.into_select_statement());
        self.into_state()
    }

    pub fn intersect(
        mut self, other: impl IntoSelectStatement<'a, Sqlite, X>,
    ) -> Select<'a, T, D, Normal, X> {
        self.statement.combine(SetOperator::Intersect, other.into_select_statement());
        self.into_state()
    }

    pub fn except(
        mut self, other: impl IntoSelectStatement<'a, Sqlite, X>,
    ) -> Select<'a, T, D, Normal, X> {
        self.statement.combine(SetOperator::Except, other.into_select_statement());
        self.into_state()
    }
}

impl<'c, 'a, T, E, TAG, X> Select<'a, T, OuterQuery<'c, E>, TAG, X>
    where
        T: Model<DB = Sqlite>,
        E: ExecutableSync<Native = rusqlite::Connection> {
    pub fn load(self) -> Result<Vec<T>> {
        self.load_as::<T>()
    }

    pub fn load_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<DB = Sqlite> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::from_row(&row))
    }

    pub fn load_in<U>(self) -> Result<Vec<U>>
        where
            for<'r> U: FromColumn<'r, Sqlite, X> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::extract(&row))
    }
}

// Inner Query

impl<'a, X> Select<'a, (), InnerQuery, Normal, X> {
    pub fn from<M: Model<DB = Sqlite>>() -> Select<'a, M, InnerQuery, JustAddFrom, X> {
        Select {
            executor: (),
            statement: SelectStatement::new(M::TABLE_NAME),
            _marker: PhantomData,
        }
    }
}

impl<'a, T: Model<DB = Sqlite>, TAG, X> Select<'a, T, InnerQuery, TAG, X> {
    pub fn alias<I: IntoIdentifier<A>, A>(self, alias: I) -> Projection<'a, Sqlite, X> {
        self.into_projection().alias(alias)
    }
}

impl<'a, T, TAG, X> TypedNode for Select<'a, T, InnerQuery, TAG, X> {
    type Output = Dynamic;
}

impl<'a, T: Model<DB = Sqlite>, Target, TAG, X> IntoOperand<'a, Target, Sqlite, Close, X>
    for Select<'a, T, InnerQuery, TAG, X> {
    fn into_operand(self) -> Operand<'a, Sqlite> {
        Operand::from_select(self.statement)
    }
}

impl<'a, T, TAG, X> IntoProjection<'a, Sqlite, X> for Select<'a, T, InnerQuery, TAG, X>
    where
        T: Model<DB = Sqlite> {
    fn into_projection(self) -> Projection<'a, Sqlite, X> {
        Projection::new(Operand::from_select(self.into_select_statement()))
    }
}

impl<'a, T, TAG, X> IntoArg<Projection<'a, Sqlite, X>, ArgAst>
    for Select<'a, T, InnerQuery, TAG, X>
    where
        T: Model<DB = Sqlite> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, Sqlite, X> {
        self.into_projection()
    }
}

impl<'a, M, D, TAG, X> IntoSrfFunction<'a, X> for Select<'a, M, D, TAG, X>
    where
        M: Model<DB = Sqlite>,
        D: Dispatch {
    #[inline(always)]
    fn into_srf_function(self) -> JsonSrfArgs<'a> {
        JsonSrfArgs { base: Operand::from_select(self.statement), path: None }
    }
}

impl<'a, T: Model<DB = Sqlite>, TAG, X> IntoSelectStatement<'a, Sqlite, X>
    for Select<'a, T, InnerQuery, TAG, X> {
    #[inline(always)]
    fn into_select_statement(self) -> SelectStatement<'a, Sqlite> {
        self.statement
    }
}

impl<'a, T, M, TAG, X> IntoInsertSource<'a, T::DB, T, X> for Select<'a, M, InnerQuery, TAG, X>
    where
        T: Model<DB = Sqlite>,
        M: Model<DB = Sqlite> {
    #[inline(always)]
    fn into_source(self) -> ValueSource<'a, Sqlite, T> {
        IntoInsertSource::<'a, T::DB, T, X>::into_source(self.into_select_statement())
    }
}
