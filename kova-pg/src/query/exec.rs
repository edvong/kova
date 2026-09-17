use crate::types::{Postgres, Row};
use deadpool_postgres::GenericClient;
use kova_core::database::Executor;
use kova_core::statement::StatementPayload;
use kova_core::types::ExecutableAsync;
use kova_core::{Error, Result};
use tokio_postgres::types::ToSql;

pub(crate) async fn execute_affected<'a, E, I, D>(
    executor: &Executor<E>, payload: I,
) -> Result<u64>
    where
        E: ExecutableAsync,
        E::Native: GenericClient + Sync,
        I: IntoIterator<Item = D> + Send,
        D: Into<StatementPayload<'a, Postgres>> {
    executor
        .with_conn_async(async move |native| {
            let mut total_affected = 0;

            for data in payload {
                let raw_data = data.into();
                let mut params: Vec<&(dyn ToSql + Sync)> =
                    Vec::with_capacity(raw_data.params.len());
                for param in &raw_data.params {
                    params.push(param as &(dyn ToSql + Sync));
                }

                total_affected += native
                    .execute(raw_data.sql.as_ref(), &params[..])
                    .await
                    .map_err(|e| Error::Database(Box::new(e)))?;
            }

            Ok(total_affected)
        })
        .await
}

pub(crate) async fn fetch_rows<'a, E, I, D, U, F>(
    executor: &Executor<E>, payload: I, mut f: F,
) -> Result<Vec<U>>
    where
        E: ExecutableAsync,
        E::Native: GenericClient + Sync,
        I: IntoIterator<Item = D> + Send,
        D: Into<StatementPayload<'a, Postgres>>,
        F: FnMut(Row<'_>) -> Result<U> + Send {
    executor
        .with_conn_async(async move |native| {
            let mut result = Vec::new();

            for data in payload {
                let raw_data = data.into();
                let mut params: Vec<&(dyn ToSql + Sync)> =
                    Vec::with_capacity(raw_data.params.len());
                for param in &raw_data.params {
                    params.push(param as &(dyn ToSql + Sync));
                }

                let rows = native
                    .query(raw_data.sql.as_ref(), &params[..])
                    .await
                    .map_err(|e| Error::Database(Box::new(e)))?;

                result.reserve(rows.len());

                for raw_row in rows {
                    let row = Row::new(&raw_row);
                    result.push(f(row)?);
                }
            }

            Ok(result)
        })
        .await
}
