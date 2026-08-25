use crate::types::{Row, Sqlite};
use kova_core::database::Executor;
use kova_core::dsl::{Condition, GroupBy, IntoArgs, Join, JoinType, Projection};
use kova_core::statement::SelectStatement;
use kova_core::types::{
    Database, ExecutableSync, FromRow, FromValue, GetColumn, GetColumns, Model,
    Select as CoreSelect,
};
use kova_core::{Error, Result};

pub struct Select<'c, 'a, T, E, DB: Database> {
    pub(crate) executor: &'c Executor<E>,
    pub(crate) statement: SelectStatement<'a, T, DB>,
}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Sqlite>
where
    T: Model<DB = Sqlite>,
{
    pub(crate) fn new(executor: &'c Executor<E>) -> Self {
        Self {
            executor,
            statement: SelectStatement::new(),
        }
    }

    pub fn columns<C>(mut self, cols: C) -> Self where C: IntoArgs<Projection<'a, Sqlite>> {
        self.statement.columns(cols);
        self
    }

    pub fn inner_join<M>(mut self, cond: Condition<'a, Sqlite>) -> Self where M: Model<DB = Sqlite> {
        let joining = Join {
            typ: JoinType::Inner,
            target_table: T::TABLE_NAME,
            on: cond,
        };
        self.statement.join(joining);
        self
    }

    pub fn left_join<M>(mut self, cond: Condition<'a, Sqlite>) -> Self where M: Model<DB = Sqlite> {
        let joining = Join {
            typ: JoinType::Left,
            target_table: T::TABLE_NAME,
            on: cond,
        };
        self.statement.join(joining);
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Sqlite>) -> Self {
        self.statement.r#where(cond);
        self
    }

    pub fn group_by<C>(mut self, cols: C) -> Self where C: IntoArgs<GroupBy<'a, Sqlite>> {
        self.statement.group_by(cols);
        self
    }

    pub fn having(mut self, cond: Condition<'a, Sqlite>) -> Self {
        self.statement.having(cond);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        if let Some(p) =self.statement.pagination_ext_mut() {
            p.limit = Some(limit);
        }
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        if let Some(p) = self.statement.pagination_ext_mut() {
            p.offset = Some(offset);
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
        self.load_as::<T>()
    }

    pub fn load_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Sqlite>,
    {
        self.process_rows(|row| U::from_row(&row))
    }

    pub fn load_scalar<U>(self) -> Result<Vec<U>>
    where
        U: FromValue<Sqlite>,
    {
        self.process_rows(|row| row.get_column(0))
    }

    pub fn load_columns<U>(self) -> Result<Vec<U>>
    where
        for<'r> Row<'r>: GetColumns<'r, U>,
    {
        self.process_rows(|row| {
            let mut idx = 0;
            // The cursor incrementation is handled by the internal GetColumns protocol
            row.get_columns(&mut idx)
        })
    }

    fn process_rows<U, F>(self, mut f: F) -> Result<Vec<U>>
    where
        F: FnMut(Row<'_>) -> Result<U>,
    {
        let data = self.statement.build();
        self.executor.with_conn_sync(|native| {
            let mut stmt = native
                .prepare(&data.sql)
                .map_err(|e| Error::Database(Box::new(e)))?;

            let mut rows = stmt
                .query(rusqlite::params_from_iter(data.params))
                .map_err(|e| Error::Database(Box::new(e)))?;

            let mut result = Vec::new();
            while let Some(raw_row) = rows.next().map_err(|e| Error::Database(Box::new(e)))? {
                let row = Row::new(raw_row);
                result.push(f(row)?);
            }
            Ok(result)
        })
    }
}

impl<'c, 'a, T, E> CoreSelect<'c, 'a, T, E, Sqlite> for Select<'c, 'a, T, E, Sqlite>
where
    T: Model<DB = Sqlite>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Select::new(executor)
    }
}
