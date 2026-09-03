use crate::query::exec::{execute_affected, fetch_rows};
use crate::query::{NoReturn, Returning};
use crate::types::Sqlite;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Condition, IntoArgs, Projection};
use kova_core::statement::DeleteStatement;
use kova_core::types::{
    Database, Delete as CoreDelete, ExecutableSync, FromColumn, FromRow, Model,
};
use std::iter::once;
use std::marker::PhantomData;

pub struct Delete<'c, 'a, T, E, DB: Database, R = NoReturn> {
    executor: &'c Executor<E>,
    statement: DeleteStatement<'a, T, DB>,
    _marker: PhantomData<R>,
}

impl<'c, 'a, T, E> CoreDelete<'c, 'a, T, E, Sqlite> for Delete<'c, 'a, T, E, Sqlite, NoReturn>
where
    T: Model<DB = Sqlite>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Self {
            executor,
            statement: DeleteStatement::new(),
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E> Delete<'c, 'a, T, E, Sqlite, NoReturn>
where
    T: Model<DB = Sqlite>,
{
    pub fn returning<P>(mut self, cols: P) -> Delete<'c, 'a, T, E, Sqlite, Returning>
    where
        P: IntoArgs<Projection<'a, Sqlite>>,
    {
        self.statement.returning(cols);
        Delete {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, R> Delete<'c, 'a, T, E, Sqlite, R>
where
    T: Model<DB = Sqlite>,
{
    pub fn filter(mut self, cond: Condition<'a, Sqlite>) -> Self {
        self.statement.r#where(cond);
        self
    }
}

impl<'c, 'a, T, E> Delete<'c, 'a, T, E, Sqlite, NoReturn>
where
    T: Model<DB = Sqlite>,
    E: ExecutableSync<Native = rusqlite::Connection>,
{
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()))
    }
}

impl<'c, 'a, T, E> Delete<'c, 'a, T, E, Sqlite, Returning>
where
    T: Model<DB = Sqlite>,
    E: ExecutableSync<Native = rusqlite::Connection>,
{
    pub fn execute_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Sqlite>,
    {
        fetch_rows(self.executor, once(self.statement.build()), |row| {
            U::from_row(&row)
        })
    }

    pub fn execute_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: FromColumn<'r, Sqlite>,
    {
        fetch_rows(self.executor, once(self.statement.build()), |row| {
            U::extract(&row)
        })
    }
}
