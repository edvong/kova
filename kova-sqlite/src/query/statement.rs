use crate::query::exec::{execute_affected, fetch_rows};
use crate::types::{Sqlite, SqliteExecutor};
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::RawStatement;
use kova_core::dsl::{Bind, IntoArg};
use kova_core::types::{DecodeColumn, FromRow, Statement as CoreStatement};
use std::borrow::Cow;
use std::iter::once;
use std::marker::PhantomData;

pub struct Statement<'c, 'a, E, X> {
    executor: &'c Executor<E>,
    statement: RawStatement<'a, Sqlite>,
    _marker: PhantomData<X>,
}

impl<'c, 'a, E, X> CoreStatement<'c, 'a, E, X> for Statement<'c, 'a, E, X> {
    fn new<S: Into<Cow<'a, str>>>(executor: &'c Executor<E>, sql: S) -> Self {
        Self { executor, statement: RawStatement::new(sql), _marker: PhantomData }
    }
}

impl<'c, 'a, E, X> Statement<'c, 'a, E, X> {
    pub fn bind<A, M>(mut self, args: A) -> Self
        where
            A: IntoArg<Bind<'a, Sqlite, X>, M> {
        self.statement.bind(args.into_args_erased());
        self
    }
}

impl<'c, 'a, E, X> Statement<'c, 'a, E, X>
    where
        E: SqliteExecutor {
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()?))
    }

    pub fn load_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<Sqlite, X> {
        fetch_rows(self.executor, once(self.statement.build()?), |row| U::from_row(&row))
    }

    pub fn load_in<U>(self) -> Result<Vec<U>>
        where
            U: DecodeColumn<Sqlite, X> {
        fetch_rows(self.executor, once(self.statement.build()?), |row| U::extract(&row))
    }
}
