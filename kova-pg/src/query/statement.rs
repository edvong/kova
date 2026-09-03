use crate::types::{Postgres, Row};
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Bind, IntoArgs};
use kova_core::statement::RawStatement;
use kova_core::types::{ExecutableAsync, FromColumn, FromRow, Statement as CoreStatement};
use std::borrow::Cow;
use tokio_postgres::types::ToSql;

pub struct Statement<'c, 'a, E> {
    executor: &'c Executor<E>,
    statement: RawStatement<'a, Postgres>,
}

impl<'c, 'a, E> Statement<'c, 'a, E> {
    pub fn bind<A>(&mut self, args: A)
    where
        A: IntoArgs<Bind<'a, Postgres>>,
    {
        self.statement.bind(args);
    }
}

impl<'c, 'a, E> Statement<'c, 'a, E>
where
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn execute(self) -> Result<u64> {
        let data = self.statement.build()?;
        self.executor
            .with_conn_async(async move |native| {
                let mut params: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(data.params.len());
                for param in &data.params {
                    params.push(param as &(dyn ToSql + Sync));
                }
                native
                    .execute(data.sql.as_ref(), &params[..])
                    .await
                    .map_err(|e| kova_core::Error::Database(Box::new(e)))
            })
            .await
    }

    pub async fn load_as<U>(self) -> Result<Vec<U>>
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

    pub async fn load_in<U>(self) -> Result<Vec<U>>
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
        let data = self.statement.build()?;
        self.executor
            .with_conn_async(async move |native| {
                let mut params: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(data.params.len());
                for param in &data.params {
                    params.push(param as &(dyn ToSql + Sync));
                }
                native
                    .query(data.sql.as_ref(), &params[..])
                    .await
                    .map_err(|e| kova_core::Error::Database(Box::new(e)))
            })
            .await
    }
}

impl<'c, 'a, E> CoreStatement<'c, 'a, E, Postgres> for Statement<'c, 'a, E> {
    fn new<S: Into<Cow<'a, str>>>(executor: &'c Executor<E>, sql: S) -> Self {
        Self {
            executor,
            statement: RawStatement::new(sql),
        }
    }
}
