use crate::query::exec::{execute_affected, fetch_rows};
use crate::types::Sqlite;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Bind, IntoArgs};
use kova_core::statement::RawStatement;
use kova_core::types::{Database, ExecutableSync, FromColumn, FromRow, Statement as CoreStatement};
use std::borrow::Cow;
use std::iter::once;
use std::marker::PhantomData;

pub struct Statement<'c, 'a, DB: Database, E, X> {
    executor: &'c Executor<E>,
    statement: RawStatement<'a, DB>,
    _marker: PhantomData<X>,
}

impl<'c, 'a, E, X> CoreStatement<'c, 'a, Sqlite, E, X> for Statement<'c, 'a, Sqlite, E, X> {
    fn new<S: Into<Cow<'a, str>>>(executor: &'c Executor<E>, sql: S) -> Self {
        Self {
            executor,
            statement: RawStatement::new(sql),
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, E, X> Statement<'c, 'a, Sqlite, E, X> {
    pub fn bind<A>(&mut self, args: A)
    where
        A: IntoArgs<Bind<'a, Sqlite>>,
    {
        self.statement.bind(args.into_args());
    }
}

impl<'c, 'a, E, X> Statement<'c, 'a, Sqlite, E, X>
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
        for<'r> U: FromColumn<'r, Sqlite, X>,
    {
        fetch_rows(self.executor, once(self.statement.build()?), |row| {
            U::extract(&row)
        })
    }
}
