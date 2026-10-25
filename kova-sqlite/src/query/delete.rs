use crate::query::exec::{execute_affected, fetch_rows};
use crate::types::{Sqlite, SqliteExecutor};
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::DeleteStatement;
use kova_core::dsl::{Condition, EraseX, IntoArg, Projection};
use kova_core::types::{DecodeColumn, Delete as CreateDelete, FromRow, Model};
use std::iter::once;
use std::marker::PhantomData;

pub struct Delete<'c, 'a, T, E, R, X> {
    executor: &'c Executor<E>,
    statement: DeleteStatement<'a, Sqlite, T>,
    _marker: PhantomData<(R, X)>,
}

pub struct NoReturn;
pub struct Returning;

pub(crate) type InitDelete<'c, 'a, T, E, X> = Delete<'c, 'a, T, E, NoReturn, X>;

impl<'c, 'a, T, E, X> CreateDelete<'c, 'a, T, E, X> for InitDelete<'c, 'a, T, E, X>
    where
        T: Model<Sqlite, X> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: DeleteStatement::new(), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X> Delete<'c, 'a, T, E, NoReturn, X>
    where
        T: Model<Sqlite, X> {
    pub fn returning<A, M>(mut self, cols: A) -> Delete<'c, 'a, T, E, Returning, X>
        where
            A: IntoArg<Projection<'a, Sqlite, X>, M> {
        self.statement.returning(cols.into_args_erased());
        Delete { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, R, X> Delete<'c, 'a, T, E, R, X>
    where
        T: Model<Sqlite, X> {
    pub fn filter(mut self, cond: Condition<'a, Sqlite, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }
}

impl<'c, 'a, T, E, X> Delete<'c, 'a, T, E, NoReturn, X>
    where
        T: Model<Sqlite, X>,
        E: SqliteExecutor {
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()?))
    }
}

impl<'c, 'a, T, E, X> Delete<'c, 'a, T, E, Returning, X>
    where
        T: Model<Sqlite, X>,
        E: SqliteExecutor {
    pub fn execute_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<Sqlite, X> {
        fetch_rows(self.executor, once(self.statement.build()?), |row| U::from_row(&row))
    }

    pub fn execute_in<U>(self) -> Result<Vec<U>>
        where
            U: DecodeColumn<Sqlite, X> {
        fetch_rows(self.executor, once(self.statement.build()?), |row| U::extract(&row))
    }
}
