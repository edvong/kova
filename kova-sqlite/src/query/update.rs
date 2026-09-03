use crate::query::exec::{execute_affected, fetch_rows};
use crate::query::{NoReturn, Returning};
use crate::types::Sqlite;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Assignment, Condition, IntoArgs, Projection};
use kova_core::statement::UpdateStatement;
use kova_core::types::{
    Database, ExecutableSync, FromColumn, FromRow, Model, Update as CoreUpdate,
};
use std::iter::once;
use std::marker::PhantomData;

pub struct Update<'c, 'a, T, E, DB: Database, R = NoReturn> {
    executor: &'c Executor<E>,
    statement: UpdateStatement<'a, T, DB>,
    _marker: PhantomData<R>,
}

impl<'c, 'a, T, E> CoreUpdate<'c, 'a, T, E, Sqlite> for Update<'c, 'a, T, E, Sqlite, NoReturn>
where
    T: Model<DB = Sqlite>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Self {
            executor,
            statement: UpdateStatement::new(),
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E> Update<'c, 'a, T, E, Sqlite, NoReturn>
where
    T: Model<DB = Sqlite>,
{
    pub fn set<A>(mut self, assignments: A) -> Self
    where
        A: IntoArgs<Assignment<'a, Sqlite>>,
    {
        self.statement.set(assignments);
        self
    }

    pub fn returning<P>(mut self, cols: P) -> Update<'c, 'a, T, E, Sqlite, Returning>
    where
        P: IntoArgs<Projection<'a, Sqlite>>,
    {
        self.statement.returning(cols);
        Update {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, R> Update<'c, 'a, T, E, Sqlite, R>
where
    T: Model<DB = Sqlite>,
{
    pub fn filter(mut self, cond: Condition<'a, Sqlite>) -> Self {
        self.statement.r#where(cond);
        self
    }
}

impl<'c, 'a, T, E> Update<'c, 'a, T, E, Sqlite, NoReturn>
where
    T: Model<DB = Sqlite>,
    E: ExecutableSync<Native = rusqlite::Connection>,
{
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()))
    }
}

impl<'c, 'a, T, E> Update<'c, 'a, T, E, Sqlite, Returning>
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
