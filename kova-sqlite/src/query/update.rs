use crate::query::exec::{execute_affected, fetch_rows};
use crate::query::{NoReturn, Returning};
use crate::types::Sqlite;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::UpdateStatement;
use kova_core::dsl::{Assignment, Condition, IntoArgs, Projection};
use kova_core::types::{
    Database, ExecutableSync, FromColumn, FromRow, Model, Update as CoreUpdate,
};
use std::iter::once;
use std::marker::PhantomData;

pub struct Update<'c, 'a, DB: Database, T, E, X, R = NoReturn> {
    executor: &'c Executor<E>,
    statement: UpdateStatement<'a, DB, T>,
    _marker: PhantomData<(X, R)>,
}

impl<'c, 'a, T, E, X> CoreUpdate<'c, 'a, Sqlite, T, E, X>
    for Update<'c, 'a, Sqlite, T, E, X, NoReturn>
    where
        T: Model<DB = Sqlite> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: UpdateStatement::new(), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, Sqlite, T, E, X, NoReturn>
    where
        T: Model<DB = Sqlite> {
    pub fn set<A>(mut self, assignments: A) -> Self
        where
            A: IntoArgs<Assignment<'a, Sqlite, X>> {
        self.statement.set(assignments);
        self
    }

    pub fn returning<A>(mut self, cols: A) -> Update<'c, 'a, Sqlite, T, E, X, Returning>
        where
            A: IntoArgs<Projection<'a, Sqlite, X>> {
        self.statement.returning(cols);

        Update { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X, R> Update<'c, 'a, Sqlite, T, E, X, R>
    where
        T: Model<DB = Sqlite> {
    pub fn filter(mut self, cond: Condition<'a, Sqlite, X>) -> Self {
        self.statement.r#where(cond);
        self
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, Sqlite, T, E, X, NoReturn>
    where
        T: Model<DB = Sqlite>,
        E: ExecutableSync<Native = rusqlite::Connection> {
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()))
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, Sqlite, T, E, X, Returning>
    where
        T: Model<DB = Sqlite>,
        E: ExecutableSync<Native = rusqlite::Connection> {
    pub fn execute_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<DB = Sqlite> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::from_row(&row))
    }

    pub fn execute_in<U>(self) -> Result<Vec<U>>
        where
            for<'r> U: FromColumn<'r, Sqlite, X> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::extract(&row))
    }
}
