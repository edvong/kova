use crate::dsl::func::IntoSrfFunction;
use crate::dsl::{LockItem, LockMode, WaitPolicy};
use crate::query::exec::fetch_rows;
use crate::types::Postgres;
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::{IntoSelectStatement, SelectStatement};
use kova_core::dsl::{
    ArgAst, Close, Condition, Dynamic, EraseX, FromItem, GroupBy, ValueSource,
    IntoArg, IntoIdentifier, IntoInsertSource, IntoJoinConstraint, IntoOperand, IntoProjection,
    JoinType, Operand, Ordering, Projection, SetOperator, TypedNode
};
use kova_core::types::{
    Cte, ExecutableAsync, DecodeColumn, FromRow, Model, Relation, Select as NewSelect
};
use std::iter::once;
use std::marker::PhantomData;

pub struct Select<'a, T, D: Dispatch, S, X> {
    pub(crate) executor: D::Executor,
    pub(crate) statement: SelectStatement<'a, Postgres>,
    _marker: PhantomData<(T, S, X)>,
}

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

pub(crate) type InnerSelect<'a, M, S, X> = Select<'a, M, InnerQuery, S, X>;
pub(crate) type InitSelect<'c, 'a, T, E, X> = Select<'a, T, OuterQuery<'c, E>, JustAddFrom, X>;

impl<'c, 'a, T, E, X> NewSelect<'c, 'a, Postgres, T, E, X> for InitSelect<'c, 'a, T, E, X>
    where
        T: Model<Postgres, X> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: SelectStatement::new(T::NAME), _marker: PhantomData }
    }
}

// Initial state: either .tag() or append .srf() to the main table.
impl<'a, T, D: Dispatch, X> Select<'a, T, D, JustAddFrom, X>
    where
        T: Model<Postgres, X> {
    pub fn tag<Marker: 'static>(mut self) -> Select<'a, T, D, TaggedFrom, X> {
        self.statement.tag_latest_from::<Marker>();
        self.into_state()
    }

    pub fn srf<A, M>(mut self, args: A) -> Select<'a, T, D, JustAddSrf, X>
        where
            A: IntoArg<FromItem<'a, Postgres, X>, M> {
        self.statement.srf(args.into_args_erased());
        self.into_state()
    }
}

// Main table tagged status: still allows appending once .srf()
impl<'a, T, D: Dispatch, X> Select<'a, T, D, TaggedFrom, X>
    where
        T: Model<Postgres, X> {
    pub fn srf<A, M>(mut self, args: A) -> Select<'a, T, D, JustAddSrf, X>
        where
            A: IntoArg<FromItem<'a, Postgres, X>, M> {
        self.statement.srf(args.into_args_erased());
        self.into_state()
    }
}

// Just appended SRF status: .tag() is allowed for this SRF, but the .srf() method no longer exists.
impl<'a, T, D: Dispatch, X> Select<'a, T, D, JustAddSrf, X>
    where
        T: Model<Postgres, X> {
    pub fn tag<Marker: 'static>(mut self) -> Select<'a, T, D, Normal, X> {
        self.statement.tag_latest_from::<Marker>();
        self.into_state()
    }
}

impl<'c, 'a, T, D: Dispatch, X> Select<'a, T, D, JustAddJoin, X>
    where
        T: Model<Postgres, X> {
    pub fn tag<Marker: 'static>(mut self) -> Select<'a, T, D, Normal, X> {
        self.statement.tag_latest_join::<Marker>();
        self.into_state()
    }
}

impl<'a, T, D: Dispatch, S, X> Select<'a, T, D, S, X> where T: Model<Postgres, X> {
    fn into_state<State>(self) -> Select<'a, T, D, State, X> {
        Select {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }

    pub fn with<C: Cte>(mut self, query: impl IntoSelectStatement<'a, Postgres, X>)
        -> Select<'a, T, D, Normal, X> {
        self.statement.with::<C>(query.into_select_statement());
        self.into_state()
    }

    pub fn distinct(mut self) -> Select<'a, T, D, Normal, X> {
        self.statement.distinct();
        self.into_state()
    }

    pub fn columns<A, M>(mut self, cols: A) -> Select<'a, T, D, Normal, X>
        where
            A: IntoArg<Projection<'a, Postgres, X>, M> {
        self.statement.columns(cols.into_args_erased());
        self.into_state()
    }

    pub fn inner_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, Postgres, X>,
    ) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.join::<R, _, X>(JoinType::Inner, constraint);
        self.into_state()
    }

    pub fn left_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, Postgres, X>,
    ) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.join::<R, _, X>(JoinType::Left, constraint);
        self.into_state()
    }

    pub fn right_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, Postgres, X>,
    ) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.join::<R, _, X>(JoinType::Right, constraint);
        self.into_state()
    }

    pub fn full_join<R: Relation>(
        mut self, constraint: impl IntoJoinConstraint<'a, Postgres, X>,
    ) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.join::<R, _, X>(JoinType::Full, constraint);
        self.into_state()
    }

    pub fn cross_join<R: Relation>(mut self) -> Select<'a, T, D, JustAddJoin, X> {
        self.statement.cross_join::<R>();
        self.into_state()
    }

    pub fn filter(mut self, cond: Condition<'a, Postgres, X>) -> Select<'a, T, D, Normal, X> {
        self.statement.r#where(cond.erase_x());
        self.into_state()
    }

    pub fn group_by<A, M>(mut self, cols: A) -> Select<'a, T, D, Normal, X>
        where
            A: IntoArg<GroupBy<'a, Postgres, X>, M> {
        self.statement.group_by(cols.into_args_erased());
        self.into_state()
    }

    pub fn having(mut self, cond: Condition<'a, Postgres, X>) -> Select<'a, T, D, Normal, X> {
        self.statement.having(cond.erase_x());
        self.into_state()
    }

    pub fn order_by<A, M>(mut self, cols: A) -> Select<'a, T, D, Normal, X>
        where
            A: IntoArg<Ordering<'a, Postgres, X>, M> {
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

    pub fn lock<M>(mut self, mode: LockMode, wait: Option<WaitPolicy>)
        -> Select<'a, T, D, Normal, X>
        where
            M: Model<Postgres, X> {
        self.statement.locking_mut().locks.push(LockItem {
            mode,
            tables: vec![M::NAME],
            wait,
        });
        self.into_state()
    }

    pub fn lock_all(mut self, mode: LockMode, wait: Option<WaitPolicy>)
        -> Select<'a, T, D, Normal, X> {
        self.statement
            .locking_mut()
            // Empty represents lock globally
            .locks
            .push(LockItem { mode, tables: vec![], wait });
        self.into_state()
    }

    pub fn union(mut self, other: impl IntoSelectStatement<'a, Postgres, X>)
        -> Select<'a, T, D, Normal, X> {
        self.statement.combine(SetOperator::Union, other.into_select_statement());
        self.into_state()
    }

    pub fn union_all(mut self, other: impl IntoSelectStatement<'a, Postgres, X>)
        -> Select<'a, T, D, Normal, X> {
        self.statement.combine(SetOperator::UnionAll, other.into_select_statement());
        self.into_state()
    }

    pub fn intersect(mut self, other: impl IntoSelectStatement<'a, Postgres, X>)
        -> Select<'a, T, D, Normal, X> {
        self.statement.combine(SetOperator::Intersect, other.into_select_statement());
        self.into_state()
    }

    pub fn except(mut self, other: impl IntoSelectStatement<'a, Postgres, X>)
        -> Select<'a, T, D, Normal, X> {
        self.statement.combine(SetOperator::Except, other.into_select_statement());
        self.into_state()
    }
}

impl<'c, 'a, T, E, S, X> Select<'a, T, OuterQuery<'c, E>, S, X>
    where
        T: Model<Postgres, X>,
        E: ExecutableAsync,
        E::Native: GenericClient + Sync {
    pub async fn load(self) -> Result<Vec<T>> {
        self.load_as::<T>().await
    }

    pub async fn load_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<Postgres, X> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::from_row(&row)).await
    }

    pub async fn load_in<U>(self) -> Result<Vec<U>>
        where
            U: DecodeColumn<Postgres, X> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::extract(&row)).await
    }
}


// Inner Query

impl<'a, X> Select<'a, (), InnerQuery, Normal, X> {
    pub fn from<M: Model<Postgres, X>>() -> Select<'a, M, InnerQuery, JustAddFrom, X> {
        Select {
            executor: (),
            statement: SelectStatement::new(M::NAME),
            _marker: PhantomData
        }
    }
}

impl<'a, T: Model<Postgres, X>, S, X> Select<'a, T, InnerQuery, S, X> {
    pub fn alias<I: IntoIdentifier<A>, A>(self, alias: I) -> Projection<'a, Postgres, X> {
        self.into_projection().alias(alias)
    }
}

impl<'a, T, S, X> TypedNode for Select<'a, T, InnerQuery, S, X> {
    type Output = Dynamic; // Inner Query return type
}

impl<'a, T: Model<Postgres, X>, Target, S, X> IntoOperand<'a, Target, Postgres, Close, X>
    for Select<'a, T, InnerQuery, S, X> {
    fn into_operand(self) -> Operand<'a, Postgres> {
        Operand::from_select(self.statement)
    }
}

impl<'a, T, S, X> IntoProjection<'a, Postgres, X> for Select<'a, T, InnerQuery, S, X>
    where
        T: Model<Postgres, X> {
    fn into_projection(self) -> Projection<'a, Postgres, X> {
        Projection::new(Operand::from_select(self.into_select_statement()))
    }
}

impl<'a, T, S, X> IntoArg<Projection<'a, Postgres, X>, ArgAst> for Select<'a, T, InnerQuery, S, X>
    where
        T: Model<Postgres, X> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, Postgres, X> {
        self.into_projection()
    }
}

impl<'a, T: Model<Postgres, X>, S, X> IntoSelectStatement<'a, Postgres, X>
    for Select<'a, T, InnerQuery, S, X> {
    #[inline(always)]
    fn into_select_statement(self) -> SelectStatement<'a, Postgres> {
        self.statement
    }
}

impl<'a, T, M, S, X> IntoInsertSource<'a, Postgres, T, X> for Select<'a, M, InnerQuery, S, X>
    where
        T: Model<Postgres, X>,
        M: Model<Postgres, X> {
    #[inline(always)]
    fn into_source(self) -> ValueSource<'a, Postgres, T> {
        IntoInsertSource::<'a, Postgres, T, X>::into_source(self.into_select_statement())
    }
}

impl<'a, M, S, X> IntoSrfFunction<'a, X> for Select<'a, M, InnerQuery, S, X>
    where
        M: Model<Postgres, X> {
    #[inline(always)]
    fn into_srf_function(self) -> Operand<'a, Postgres> {
        Operand::from_select(self.statement)
    }
}

