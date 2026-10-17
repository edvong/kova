use crate::query::exec::{execute_affected, fetch_rows};
use crate::query::{NoReturn, Returning};
use crate::types::Sqlite;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::UpdateStatement;
use kova_core::dsl::{Assignment, Condition, EraseX, IntoArg, IntoRelationList, Projection};
use kova_core::types::{
    ExecutableSync, FromColumn, FromRow, Model, Update as CoreUpdate,
};
use std::iter::once;
use std::marker::PhantomData;

pub struct Update<'c, 'a, T, E, X, R = NoReturn> {
    executor: &'c Executor<E>,
    statement: UpdateStatement<'a, Sqlite, T>,
    _marker: PhantomData<(X, R)>,
}

impl<'c, 'a, T, E, X> CoreUpdate<'c, 'a, T, E, X>
    for Update<'c, 'a, T, E, X, NoReturn>
    where
        T: Model<DB = Sqlite> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: UpdateStatement::new(), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, X, NoReturn>
    where
        T: Model<DB = Sqlite> {
    pub fn set<A, M>(mut self, assignments: A) -> Self
        where
            A: IntoArg<Assignment<'a, Sqlite, X>, M> {
        self.statement.set(assignments.into_args_erased());
        self
    }

    pub fn returning<A, M>(mut self, cols: A) -> Update<'c, 'a, T, E, X, Returning>
        where
            A: IntoArg<Projection<'a, Sqlite, X>, M> {
        self.statement.returning(cols.into_args_erased());

        Update { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X, R> Update<'c, 'a, T, E, X, R>
    where
        T: Model<DB = Sqlite> {
    pub fn from<REL: IntoRelationList>(mut self) -> Self {
        self.statement.from::<REL>();
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Sqlite, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, X, NoReturn>
    where
        T: Model<DB = Sqlite>,
        E: ExecutableSync<Native = rusqlite::Connection> {
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()))
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, X, Returning>
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
