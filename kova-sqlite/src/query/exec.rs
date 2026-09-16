use crate::types::{Row, Sqlite};
use kova_core::database::Executor;
use kova_core::statement::StatementPayload;
use kova_core::types::ExecutableSync;
use kova_core::{Error, Result};

pub(crate) fn execute_affected<'a, E, I, D>(executor: &Executor<E>, payload: I) -> Result<u64>
    where
        E: ExecutableSync<Native = rusqlite::Connection>,
        I: IntoIterator<Item = D>,
        D: Into<StatementPayload<'a, Sqlite>>, {
    executor.with_conn_sync(|native| {
        let mut total_affected = 0;

        for data in payload {
            let raw_data = data.into();
            let mut stmt =
                native.prepare(raw_data.sql.as_ref()).map_err(|e| Error::Database(Box::new(e)))?;

            total_affected += stmt
                .execute(rusqlite::params_from_iter(raw_data.params))
                .map_err(|e| Error::Database(Box::new(e)))?;
        }

        Ok(total_affected as u64)
    })
}

pub(crate) fn fetch_rows<'a, E, I, D, U, F>(
    executor: &Executor<E>, payload: I, mut f: F,
) -> Result<Vec<U>>
    where
        E: ExecutableSync<Native = rusqlite::Connection>,
        I: IntoIterator<Item = D>,
        D: Into<StatementPayload<'a, Sqlite>>,
        F: FnMut(Row<'_>) -> Result<U>, {
    executor.with_conn_sync(|native| {
        let mut result = Vec::with_capacity(32);

        for data in payload {
            let raw_data = data.into();
            let mut stmt =
                native.prepare(raw_data.sql.as_ref()).map_err(|e| Error::Database(Box::new(e)))?;

            let mut rows = stmt
                .query(rusqlite::params_from_iter(raw_data.params))
                .map_err(|e| Error::Database(Box::new(e)))?;

            while let Some(raw_row) = rows.next().map_err(|e| Error::Database(Box::new(e)))? {
                let row = Row::new(raw_row);
                result.push(f(row)?);
            }
        }

        Ok(result)
    })
}
