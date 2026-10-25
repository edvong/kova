use crate::query::exec::{execute_affected, fetch_rows};
use crate::types::{Sqlite, SqliteExecutor};
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::InsertStatement;
use kova_core::dsl::{
    Assignment, Condition, EraseX, IntoArg, IntoConflictTarget, IntoInsertSource, Projection,
};
use kova_core::types::{DecodeColumn, FromRow, Insert as NewInsert, Model};
use std::marker::PhantomData;

pub struct NoConflict;
pub struct ConflictTarget;
pub struct ConflictNothing;
pub struct ConflictUpdate;

pub trait CanExecute {}
impl CanExecute for NoConflict {}
impl CanExecute for ConflictNothing {}
impl CanExecute for ConflictUpdate {}

pub struct NoReturn;
pub struct Returning;

pub struct Insert<'c, 'a, T, E, R, C, X> {
    executor: &'c Executor<E>,
    statement: InsertStatement<'a, Sqlite, T, X>,
    _marker: PhantomData<(R, C)>,
}

pub(crate) type InitInsert<'c, 'a, T, E, X> = Insert<'c, 'a, T, E, NoReturn, NoConflict, X>;

impl<'c, 'a, T, E, X> NewInsert<'c, 'a, Sqlite, T, E, X> for InitInsert<'c, 'a, T, E, X>
    where
        T: Model<Sqlite, X> {
    fn new<D>(executor: &'c Executor<E>, data: D) -> Self
        where
            D: IntoInsertSource<'a, Sqlite, T, X> {
        Self { executor, statement: InsertStatement::new(data), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, C, X> Insert<'c, 'a, T, E, NoReturn, C, X>
    where
        T: Model<Sqlite, X> {
    pub fn returning<A, M>(mut self, cols: A) -> Insert<'c, 'a, T, E, Returning, C, X>
        where
            A: IntoArg<Projection<'a, Sqlite, X>, M> {
        self.statement.returning(cols.into_args_erased());
        Insert { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, R, X> Insert<'c, 'a, T, E, R, NoConflict, X>
    where
        T: Model<Sqlite, X> {
    pub fn on_conflict<A, M>(mut self, targets: A) -> Insert<'c, 'a, T, E, R, ConflictTarget, X>
        where
            // A: IntoArg<Conflict<'a, Sqlite, X>, M> {
        A: IntoConflictTarget<'a, Sqlite, M, X>, {
        self.statement.conflict_mut().set_target(targets.into_conflict_target());
        Insert { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, R, X> Insert<'c, 'a, T, E, R, ConflictTarget, X>
    where
        T: Model<Sqlite, X> {
    pub fn do_nothing(mut self) -> Insert<'c, 'a, T, E, R, ConflictNothing, X> {
        self.statement.conflict_mut().set_do_nothing();
        Insert { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }

    pub fn update<A, M>(mut self, assignments: A) -> Insert<'c, 'a, T, E, R, ConflictUpdate, X>
        where
            A: IntoArg<Assignment<'a, Sqlite, X>, M> {
        self.statement.conflict_mut().set_do_update(assignments.into_args_erased());
        Insert { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, R, X> Insert<'c, 'a, T, E, R, ConflictUpdate, X>
    where
        T: Model<Sqlite, X> {
    pub fn filter(mut self, cond: Condition<'a, Sqlite, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }
}

impl<'c, 'a, T, E, C: CanExecute, X> Insert<'c, 'a, T, E, NoReturn, C, X>
    where
        T: Model<Sqlite, X>,
        E: SqliteExecutor {
    pub fn execute(self) -> Result<u64> {
        execute_affected(self.executor, self.statement.build()?)
    }
}

impl<'c, 'a, T, E, C: CanExecute, X> Insert<'c, 'a, T, E, Returning, C, X>
    where
        T: Model<Sqlite, X>,
        E: SqliteExecutor {
    pub fn execute_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<Sqlite, X> {
        fetch_rows(self.executor, self.statement.build()?, |row| U::from_row(&row))
    }

    pub fn execute_in<U>(self) -> Result<Vec<U>>
        where
            U: DecodeColumn<Sqlite, X> {
        fetch_rows(self.executor, self.statement.build()?, |row| U::extract(&row))
    }
}
