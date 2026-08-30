use crate::dsl::{LockItem, LockMode, Locking, Pagination, WaitPolicy};
use crate::types::{Postgres, Row};
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Condition, GroupBy, IntoArgs, Join, JoinType, Ordering, Projection};
use kova_core::statement::SelectStatement;
use kova_core::types::{
    Database, DecodeColumn, ExecutableAsync, FromRow, Model, Select as CoreSelect,
};
use tokio_postgres::types::ToSql;

pub struct Select<'c, 'a, T, E, DB: Database> {
    pub(crate) executor: &'c Executor<E>,
    pub(crate) statement: SelectStatement<'a, T, DB>,
}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Postgres>
where
    T: Model<DB = Postgres>,
{
    pub(crate) fn new(executor: &'c Executor<E>) -> Self {
        Self {
            executor,
            statement: SelectStatement::new(),
        }
    }

    pub fn columns<C>(mut self, cols: C) -> Self
    where
        C: IntoArgs<Projection<'a, Postgres>>,
    {
        self.statement.columns(cols);
        self
    }

    pub fn inner_join<M>(mut self, cond: Condition<'a, Postgres>) -> Self
    where
        M: Model<DB = Postgres>,
    {
        let joining = Join {
            typ: JoinType::Inner,
            target_table: T::TABLE_NAME,
            on: cond,
        };
        self.statement.join(joining);
        self
    }

    pub fn left_join<M>(mut self, cond: Condition<'a, Postgres>) -> Self
    where
        M: Model<DB = Postgres>,
    {
        let joining = Join {
            typ: JoinType::Left,
            target_table: T::TABLE_NAME,
            on: cond,
        };
        self.statement.join(joining);
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Postgres>) -> Self {
        self.statement.r#where(cond);
        self
    }

    pub fn group_by<C>(mut self, cols: C) -> Self
    where
        C: IntoArgs<GroupBy<'a, Postgres>>,
    {
        self.statement.group_by(cols);
        self
    }

    pub fn having(mut self, cond: Condition<'a, Postgres>) -> Self {
        self.statement.having(cond);
        self
    }

    pub fn order_by<C>(mut self, cols: C) -> Self
    where
        C: IntoArgs<Ordering<'a, Postgres>>,
    {
        self.statement.order_by(cols);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        let ext = self
            .statement
            .pagination_ext_mut_or_with(|| Pagination::default());
        ext.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        let ext = self
            .statement
            .pagination_ext_mut_or_with(|| Pagination::default());
        ext.offset = Some(offset);
        self
    }

    pub fn lock<M>(mut self, mode: LockMode, wait: Option<WaitPolicy>) -> Self
    where
        M: Model<DB = Postgres>,
    {
        let ext = self
            .statement
            .locking_ext_mut_or_with(|| Locking::default());
        ext.locks.push(LockItem {
            mode,
            tables: vec![M::TABLE_NAME],
            wait,
        });

        self
    }

    pub fn lock_all(mut self, mode: LockMode, wait: Option<WaitPolicy>) -> Self {
        let ext = self
            .statement
            .locking_ext_mut_or_with(|| Locking::default());
        ext.locks.push(LockItem {
            mode,
            tables: vec![], // Empty represents lock globally
            wait,
        });

        self
    }
}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Postgres>
where
    T: Model<DB = Postgres>,
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn load(self) -> Result<Vec<T>> {
        self.load_as::<T>().await
    }

    pub async fn load_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Postgres>,
    {
        let raw_rows = self.load_raw_rows().await?;
        let mut result = Vec::with_capacity(raw_rows.len());
        for raw_row in raw_rows {
            let row = Row::new(&raw_row);
            result.push(U::from_row(&row)?);
        }
        Ok(result)
    }

    pub async fn load_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: DecodeColumn<'r, Postgres>,
    {
        let raw_rows = self.load_raw_rows().await?;
        let mut result = Vec::with_capacity(raw_rows.len());
        for raw_row in raw_rows {
            let row = Row::new(&raw_row);
            result.push(U::decode_column(&row)?);
        }
        Ok(result)
    }

    async fn load_raw_rows(self) -> Result<Vec<tokio_postgres::Row>> {
        let data = self.statement.build();
        self.executor
            .with_conn_async(async move |native| {
                let mut params: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(data.params.len());
                for param in &data.params {
                    params.push(param as &(dyn ToSql + Sync));
                }

                native
                    .query(&data.sql, &params[..])
                    .await
                    .map_err(|e| kova_core::Error::Database(Box::new(e)))
            })
            .await
    }
}

impl<'c, 'a, T, E> CoreSelect<'c, 'a, T, E, Postgres> for Select<'c, 'a, T, E, Postgres>
where
    T: Model<DB = Postgres>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Select::new(executor)
    }
}
