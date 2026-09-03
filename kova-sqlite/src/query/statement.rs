use crate::query::exec::{execute_affected, fetch_rows};
use crate::types::Sqlite;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Bind, IntoArgs};
use kova_core::statement::RawStatement;
use kova_core::types::{ExecutableSync, FromColumn, FromRow, Statement as CoreStatement};
use std::borrow::Cow;
use std::iter::once;

pub struct Statement<'c, 'a, E> {
    executor: &'c Executor<E>,
    statement: RawStatement<'a, Sqlite>,
}

impl<'c, 'a, E> CoreStatement<'c, 'a, E, Sqlite> for Statement<'c, 'a, E> {
    fn new<S: Into<Cow<'a, str>>>(executor: &'c Executor<E>, sql: S) -> Self {
        Self {
            executor,
            statement: RawStatement::new(sql),
        }
    }
}

impl<'c, 'a, E> Statement<'c, 'a, E> {
    pub fn bind<A>(&mut self, args: A)
    where
        A: IntoArgs<Bind<'a, Sqlite>>,
    {
        self.statement.bind(args);
    }
}

impl<'c, 'a, E> Statement<'c, 'a, E>
where
    E: ExecutableSync<Native = rusqlite::Connection>,
{
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()?))
    }

    pub fn load_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Sqlite>,
    {
        fetch_rows(self.executor, once(self.statement.build()?), |row| {
            U::from_row(&row)
        })
    }

    pub fn load_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: FromColumn<'r, Sqlite>,
    {
        fetch_rows(self.executor, once(self.statement.build()?), |row| {
            U::extract(&row)
        })
    }
}
