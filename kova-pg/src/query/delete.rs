use crate::query::{NoReturn, Returning};
use crate::types::{Postgres, Row};
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Condition, IntoArgs, Projection};
use kova_core::statement::DeleteStatement;
use kova_core::types::{
    Database, Delete as CoreDelete, ExecutableAsync, FromColumn, FromRow, Model,
};
use std::marker::PhantomData;
use tokio_postgres::types::ToSql;

pub struct Delete<'c, 'a, T, E, DB: Database, R = NoReturn> {
    executor: &'c Executor<E>,
    statement: DeleteStatement<'a, T, DB>,
    _marker: PhantomData<R>,
}

impl<'c, 'a, T, E> Delete<'c, 'a, T, E, Postgres, NoReturn>
where
    T: Model<DB = Postgres>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Self {
            executor,
            statement: DeleteStatement::new(),
            _marker: PhantomData,
        }
    }

    pub fn returning<P>(mut self, cols: P) -> Delete<'c, 'a, T, E, Postgres, Returning>
    where
        P: IntoArgs<Projection<'a, Postgres>>,
    {
        self.statement.returning(cols);
        Delete {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, R> Delete<'c, 'a, T, E, Postgres, R>
where
    T: Model<DB = Postgres>,
{
    pub fn filter(mut self, cond: Condition<'a, Postgres>) -> Self {
        self.statement.r#where(cond);
        self
    }
}

impl<'c, 'a, T, E> Delete<'c, 'a, T, E, Postgres, NoReturn>
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

impl<'c, 'a, T, E> Delete<'c, 'a, T, E, Postgres, Returning>
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

impl<'c, 'a, T, E> CoreDelete<'c, 'a, T, E, Postgres> for Delete<'c, 'a, T, E, Postgres, NoReturn>
where
    T: Model<DB = Postgres>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Self::new(executor)
    }
}
