use crate::dsl::{LockItem, LockMode, Locking, Pagination, WaitPolicy};
use crate::query::exec::fetch_rows;
use crate::types::Postgres;
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Condition, EraseX, GroupBy, IntoArgs, Join, JoinType, Ordering, Projection};
use kova_core::statement::SelectStatement;
use kova_core::types::{
    Database, ExecutableAsync, FromColumn, FromRow, Model, Select as CoreSelect,
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
    pub fn columns<C>(mut self, cols: C) -> Self
        where
            C: IntoArgs<Projection<'a, Postgres, X>> {
        self.statement.columns(cols.into_mapped_args(|p| p.erase_x()));
        self
    }

    pub fn inner_join<M>(mut self, cond: Condition<'a, Postgres, X>) -> Self
        where
            M: Model<DB = Postgres> {
        let join = Join { typ: JoinType::Inner, target_table: T::TABLE_NAME, on: cond.erase_x() };
        self.statement.join(join);
        self
    }

    pub fn left_join<M>(mut self, cond: Condition<'a, Postgres, X>) -> Self
        where
            M: Model<DB = Postgres> {
        let join = Join { typ: JoinType::Left, target_table: T::TABLE_NAME, on: cond.erase_x() };
        self.statement.join(join);
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }

    pub fn group_by<C>(mut self, cols: C) -> Self
        where
            C: IntoArgs<GroupBy<'a, Postgres, X>> {
        self.statement.group_by(cols.into_mapped_args(|g| g.erase_x()));
        self
    }

    pub fn having(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.having(cond.erase_x());
        self
    }

    pub fn order_by<C>(mut self, cols: C) -> Self
        where
            C: IntoArgs<Ordering<'a, Postgres, X>> {
        self.statement.order_by(cols.into_mapped_args(|o| o.erase_x()));
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        let ext = self.statement.pagination_ext_mut_or_with(|| Pagination::default());
        ext.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        let ext = self.statement.pagination_ext_mut_or_with(|| Pagination::default());
        ext.offset = Some(offset);
        self
    }

    pub fn lock<M>(mut self, mode: LockMode, wait: Option<WaitPolicy>) -> Self
        where
            M: Model<DB = Postgres> {
        let ext = self.statement.locking_ext_mut_or_with(|| Locking::default());
        ext.locks.push(LockItem { mode, tables: vec![M::TABLE_NAME], wait });

        self
    }

    pub fn lock_all(mut self, mode: LockMode, wait: Option<WaitPolicy>) -> Self {
        let ext = self.statement.locking_ext_mut_or_with(|| Locking::default());
        ext.locks.push(LockItem {
            mode,
            tables: vec![], // Empty represents lock globally
            wait,
        });

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
