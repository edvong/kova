use crate::dsl::{LockItem, LockMode, WaitPolicy};
use crate::query::exec::fetch_rows;
use crate::types::Postgres;
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::{IntoSelectStatement, SelectStatement};
use kova_core::dsl::{Condition, GroupBy, IntoArgs, JoinType, Ordering, Projection};
use kova_core::types::{
    Cte, Database, ExecutableAsync, FromColumn, FromRow, Model, Relation, Select as CoreSelect,
};
use std::iter::once;
use std::marker::PhantomData;

pub struct Select<'c, 'a, DB: Database, T, E, X> {
    pub(crate) executor: &'c Executor<E>,
    pub(crate) statement: SelectStatement<'a, DB>,
    _marker: PhantomData<(T, X)>,
}

impl<'c, 'a, T, E, X> CoreSelect<'c, 'a, Postgres, T, E, X> for Select<'c, 'a, Postgres, T, E, X>
    where
        T: Model<DB = Postgres> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: SelectStatement::new(T::TABLE_NAME), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X> Select<'c, 'a, Postgres, T, E, X>
    where
        T: Model<DB = Postgres> {
    pub fn with<C: Cte>(mut self, query: impl IntoSelectStatement<'a, Postgres, X>) -> Self {
        self.statement.with::<C>(query.into_select_statement());
        self
    }

    pub fn columns<A: IntoArgs<Projection<'a, Postgres, X>>>(mut self, cols: A) -> Self {
        self.statement.columns(cols);
        self
    }

    pub fn inner_join<R: Relation>(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.join::<R, X>(JoinType::Inner, cond);
        self
    }

    pub fn left_join<R: Relation>(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.join::<R, X>(JoinType::Left, cond);
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.r#where(cond);
        self
    }

    pub fn group_by<A: IntoArgs<GroupBy<'a, Postgres, X>>>(mut self, cols: A) -> Self {
        self.statement.group_by(cols);
        self
    }

    pub fn having(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.having(cond);
        self
    }

    pub fn order_by<A: IntoArgs<Ordering<'a, Postgres, X>>>(mut self, cols: A) -> Self {
        self.statement.order_by(cols);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.statement.limit(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.statement.offset(offset);
        self
    }

    pub fn lock<M>(mut self, mode: LockMode, wait: Option<WaitPolicy>) -> Self
        where
            M: Model<DB = Postgres> {
        self.statement.locking_mut().locks.push(LockItem {
            mode,
            tables: vec![M::TABLE_NAME],
            wait,
        });
        self
    }

    pub fn lock_all(mut self, mode: LockMode, wait: Option<WaitPolicy>) -> Self {
        self.statement
            .locking_mut()
            // Empty represents lock globally
            .locks
            .push(LockItem { mode, tables: vec![], wait });
        self
    }
}

impl<'c, 'a, T, E, X> Select<'c, 'a, Postgres, T, E, X>
    where
        T: Model<DB = Postgres>,
        E: ExecutableAsync,
        E::Native: GenericClient + Sync {
    pub async fn load(self) -> Result<Vec<T>> {
        self.load_as::<T>().await
    }

    pub async fn load_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<DB = Postgres> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::from_row(&row)).await
    }

    pub async fn load_in<U>(self) -> Result<Vec<U>>
        where
            for<'r> U: FromColumn<'r, Postgres, X> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::extract(&row)).await
    }
}
