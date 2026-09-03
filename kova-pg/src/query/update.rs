use crate::query::{NoReturn, Returning};
use crate::types::{Postgres, Row};
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Assignment, Condition, IntoArgs, Projection};
use kova_core::statement::UpdateStatement;
use kova_core::types::{
    Database, ExecutableAsync, FromColumn, FromRow, Model, Update as CoreUpdate,
};
use std::marker::PhantomData;
use tokio_postgres::types::ToSql;

pub struct Update<'c, 'a, T, E, DB: Database, R = NoReturn> {
    executor: &'c Executor<E>,
    statement: UpdateStatement<'a, T, DB>,
    _marker: PhantomData<R>,
}

impl<'c, 'a, T, E> Update<'c, 'a, T, E, Postgres, NoReturn>
where
    T: Model<DB = Postgres>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Self {
            executor,
            statement: UpdateStatement::new(),
            _marker: PhantomData,
        }
    }

    pub fn set<A>(mut self, assignments: A) -> Self
    where
        A: IntoArgs<Assignment<'a, Postgres>>,
    {
        self.statement.set(assignments);
        self
    }

    pub fn returning<P>(mut self, cols: P) -> Update<'c, 'a, T, E, Postgres, Returning>
    where
        P: IntoArgs<Projection<'a, Postgres>>,
    {
        self.statement.returning(cols);
        Update {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, R> Update<'c, 'a, T, E, Postgres, R>
where
    T: Model<DB = Postgres>,
{
    pub fn filter(mut self, cond: Condition<'a, Postgres>) -> Self {
        self.statement.r#where(cond);
        self
    }
}

impl<'c, 'a, T, E> Update<'c, 'a, T, E, Postgres, NoReturn>
where
    T: Model<DB = Postgres>,
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn execute(self) -> Result<u64> {
        let data = self.statement.build();
        self.executor
            .with_conn_async(async move |native| {
                let mut params: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(data.params.len());
                for param in &data.params {
                    params.push(param as &(dyn ToSql + Sync));
                }
                native
                    .execute(&data.sql, &params[..])
                    .await
                    .map_err(|e| kova_core::Error::Database(Box::new(e)))
            })
            .await
    }
}

impl<'c, 'a, T, E> Update<'c, 'a, T, E, Postgres, Returning>
where
    T: Model<DB = Postgres>,
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn execute_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Postgres>,
    {
        let raw_rows = self.execute_raw_rows().await?;
        let mut result = Vec::with_capacity(raw_rows.len());
        for raw_row in raw_rows {
            let row = Row::new(&raw_row);
            result.push(U::from_row(&row)?);
        }
        Ok(result)
    }

    pub async fn execute_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: FromColumn<'r, Postgres>,
    {
        let raw_rows = self.execute_raw_rows().await?;
        let mut result = Vec::with_capacity(raw_rows.len());
        for raw_row in raw_rows {
            let row = Row::new(&raw_row);
            result.push(U::extract(&row)?);
        }
        Ok(result)
    }

    async fn execute_raw_rows(self) -> Result<Vec<tokio_postgres::Row>> {
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

impl<'c, 'a, T, E> CoreUpdate<'c, 'a, T, E, Postgres> for Update<'c, 'a, T, E, Postgres, NoReturn>
where
    T: Model<DB = Postgres>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Self::new(executor)
    }
}
