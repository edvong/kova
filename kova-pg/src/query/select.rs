use crate::types::{Postgres, Row};
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Condition, Pagination};
use kova_core::statement::SelectStatement;
use kova_core::types::{Database, ExecutableAsync, Model};

pub struct Select<'c, 'a, T, E, DB: Database> {
    pub(crate) executor: &'c Executor<E>,
    pub(crate) statement: SelectStatement<'a, T, DB>,
}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Postgres> where T: Model<DB = Postgres> {
    pub(crate) fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: SelectStatement::new() }
    }

    pub fn filter(mut self, cond: Condition<'a, Postgres>) -> Self {
        self.statement.filter(cond);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        if let Pagination::<'_, Postgres>::Extension(ext) = self.statement.pagination
            .get_or_insert_with(|| Pagination::<'_, Postgres>::Extension(crate::dsl::Pagination::default()))
        {
            ext.limit = Some(limit);
        }
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        if let Pagination::<'_, Postgres>::Extension(ext) = self.statement.pagination
            .get_or_insert_with(|| Pagination::<'_, Postgres>::Extension(crate::dsl::Pagination::default()))
        {
            ext.offset = Some(offset);
        }
        self
    }

}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Postgres> where
    T: Model<DB = Postgres>,
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn load(self) -> Result<Vec<T>> {
        let data = self.statement.build();
        self.executor.with_conn_async(async move |native| {
            let mut pg_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
                Vec::with_capacity(data.params.len());

            for param in &data.params {
                pg_params.push(param as &(dyn tokio_postgres::types::ToSql + Sync));
            }

            let rows = native
                .query(&data.sql, &pg_params[..])
                .await
                .map_err(|e| kova_core::Error::Database(Box::new(e)))?;

            let mut result = Vec::with_capacity(rows.len());
            for row in rows {
                let kova_row = Row::new(&row);
                result.push(T::from_row(&kova_row)?);
            }

            Ok(result)
        }).await
    }
}