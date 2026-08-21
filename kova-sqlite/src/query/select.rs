use crate::types::{Row, Sqlite};
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Condition, Pagination};
use kova_core::statement::SelectStatement;
use kova_core::types::{Database, ExecutableSync, Model};

pub struct Select<'c, 'a, T, E, DB: Database> {
    pub(crate) executor: &'c Executor<E>,
    pub(crate) statement: SelectStatement<'a, T, DB>,
}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Sqlite> where T: Model<DB = Sqlite> {
    pub(crate) fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: SelectStatement::new() }
    }

    pub fn filter(mut self, cond: Condition<'a, Sqlite>) -> Self {
        self.statement.filter(cond);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        if let Pagination::<'_, Sqlite>::Extension(ext) = self.statement.pagination
            .get_or_insert_with(|| Pagination::<'_, Sqlite>::Extension(crate::dsl::Pagination::default()))
        {
            ext.limit = Some(limit);
        }
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        if let Pagination::<'_, Sqlite>::Extension(ext) = self.statement.pagination
            .get_or_insert_with(|| Pagination::<'_, Sqlite>::Extension(crate::dsl::Pagination::default()))
        {
            ext.offset = Some(offset);
        }
        self
    }




}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Sqlite>
where
    T: Model<DB = Sqlite>,
    E: ExecutableSync<Native = rusqlite::Connection>,
{
    pub fn load(self) -> Result<Vec<T>> {
        let data = self.statement.build();
        self.executor.with_conn_sync(|native| {
            let mut stmt = native
                .prepare(&data.sql)
                .map_err(|e| kova_core::Error::Database(Box::new(e)))?;

            let mut rows = stmt
                .query(rusqlite::params_from_iter(data.params))
                .map_err(|e| kova_core::Error::Database(Box::new(e)))?;

            let mut result = Vec::new();
            while let Some(row) = rows.next().map_err(|e| kova_core::Error::Database(Box::new(e)))? {
                let kova_row = Row::new(row);
                result.push(T::from_row(&kova_row)?);
            }
            Ok(result)
        })
    }
}