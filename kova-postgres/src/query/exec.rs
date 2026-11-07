use crate::types::conn::PgClient;
use crate::types::{PgExecutor, Postgres, Row};
use kova_kernel::Result;
use kova_kernel::database::Executor;
use kova_kernel::dsl::statement::StatementPayload;

pub(crate) async fn execute_affected<'a, E, I, D>(executor: &Executor<E>, payload: I) -> Result<u64>
    where
        E: PgExecutor,
        I: IntoIterator<Item = D> + Send,
        D: Into<StatementPayload<'a, Postgres>> {
    executor
        .with_conn_async(async move |native| {
            let mut total_affected = 0;

            for data in payload {
                let raw_data = data.into();
                executor.inner().inspect(raw_data.sql.as_ref(), &raw_data.params);
                total_affected += native
                    .execute_cached(raw_data.sql.as_ref(), &raw_data.params)
                    .await?;
            }

            Ok(total_affected)
        })
        .await
}

pub(crate) async fn fetch_rows<'a, E, I, D, U, F>(executor: &Executor<E>, payload: I, mut f: F) -> Result<Vec<U>>
    where
        E: PgExecutor,
        I: IntoIterator<Item = D> + Send,
        D: Into<StatementPayload<'a, Postgres>>,
        F: FnMut(Row<'_>) -> Result<U> + Send {
    executor
        .with_conn_async(async move |native| {
            let mut result = Vec::new();

            for data in payload {
                let raw_data = data.into();
                executor.inner().inspect(raw_data.sql.as_ref(), &raw_data.params);
                let rows = native
                    .query_cached(raw_data.sql.as_ref(), &raw_data.params)
                    .await?;

                result.reserve(rows.len());
                for raw_row in &rows {
                    result.push(f(Row::new(raw_row))?);
                }
            }

            Ok(result)
        })
        .await
}