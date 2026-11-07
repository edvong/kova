use crate::types::{Row, Sqlite, SqliteExecutor};
use kova_kernel::database::Executor;
use kova_kernel::dsl::statement::StatementPayload;
use kova_kernel::{Error, Result};

pub(crate) fn execute_affected<'a, E, I, D>(executor: &Executor<E>, payload: I) -> Result<u64>
    where
        E: SqliteExecutor,
        I: IntoIterator<Item = D>,
        D: Into<StatementPayload<'a, Sqlite>> {
    executor.with_conn_sync(|native| {
        let mut total_affected = 0;

        for data in payload {
            let raw_data = data.into();
            executor.inner().inspect(raw_data.sql.as_ref(), &raw_data.params);

            let mut stmt = native
                .prepare_cached(raw_data.sql.as_ref())
                .map_err(Error::database)?;
            total_affected += stmt
                .execute(rusqlite::params_from_iter(raw_data.params))
                .map_err(Error::database)?;
        }

        Ok(total_affected as u64)
    })
}

pub(crate) fn fetch_rows<'a, E, I, D, U, F>(executor: &Executor<E>, payload: I, mut f: F) -> Result<Vec<U>>
    where
        E: SqliteExecutor,
        I: IntoIterator<Item = D>,
        D: Into<StatementPayload<'a, Sqlite>>,
        F: FnMut(Row<'_>) -> Result<U> {
    executor.with_conn_sync(|native| {
        let mut result = Vec::with_capacity(32);

        for data in payload {
            let raw_data = data.into();
            executor.inner().inspect(raw_data.sql.as_ref(), &raw_data.params);

            let mut stmt = native
                .prepare_cached(raw_data.sql.as_ref())
                .map_err(Error::database)?;
            let mut rows = stmt
                .query(rusqlite::params_from_iter(raw_data.params))
                .map_err(Error::database)?;

            while let Some(raw_row) = rows.next().map_err(Error::database)? {
                let row = Row::new(raw_row);
                result.push(f(row)?);
            }
        }

        Ok(result)
    })
}