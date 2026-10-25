use crate::query::exec::{execute_affected, fetch_rows};
use crate::types::{Sqlite, SqliteExecutor};
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::UpdateStatement;
use kova_core::dsl::{Assignment, Condition, EraseX, IntoArg, IntoRelationList, Projection};
use kova_core::types::{DecodeColumn, FromRow, Model, Update as NewUpdate};
use std::iter::once;
use std::marker::PhantomData;

pub struct Update<'c, 'a, T, E, R, S, X> {
    executor: &'c Executor<E>,
    statement: UpdateStatement<'a, Sqlite, T>,
    _marker: PhantomData<(R, S, X)>,
}

pub struct Unset;
pub struct Assigned;

pub struct NoReturn;
pub struct Returning;

pub(crate) type InitUpdate<'c, 'a, T, E, X> = Update<'c, 'a, T, E, NoReturn, Unset, X>;

impl<'c, 'a, T, E, X> NewUpdate<'c, 'a, T, E, X> for InitUpdate<'c, 'a, T, E, X>
    where
        T: Model<Sqlite, X> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: UpdateStatement::new(), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, S, X> Update<'c, 'a, T, E, NoReturn, S, X>
    where
        T: Model<Sqlite, X> {
    pub fn set<A, M>(mut self, assignments: A) -> Update<'c, 'a, T, E, NoReturn, Assigned, X>
        where
            A: IntoArg<Assignment<'a, Sqlite, X>, M> {
        self.statement.set(assignments.into_args_erased());
        Update {executor: self.executor, statement: self.statement, _marker: PhantomData}
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, NoReturn, Assigned, X>
where
    T: Model<Sqlite, X> {
    pub fn returning<A, M>(mut self, cols: A) -> Update<'c, 'a, T, E, Returning, Assigned, X>
    where
        A: IntoArg<Projection<'a, Sqlite, X>, M> {
        self.statement.returning(cols.into_args_erased());
        Update { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, R, S, X> Update<'c, 'a, T, E, R, S, X>
    where
        T: Model<Sqlite, X> {
    pub fn from<REL: IntoRelationList>(mut self) -> Self {
        self.statement.from::<REL>();
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Sqlite, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, NoReturn, Assigned, X>
    where
        T: Model<Sqlite, X>,
        E: SqliteExecutor {
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()?))
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, Returning, Assigned, X>
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
