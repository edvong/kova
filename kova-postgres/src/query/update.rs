use crate::query::exec::{execute_affected, fetch_rows};
use crate::types::{PgExecutor, Postgres};
use kova_kernel::Result;
use kova_kernel::database::Executor;
use kova_kernel::dsl::statement::UpdateStatement;
use kova_kernel::dsl::{Assignment, Condition, EraseX, IntoArg, IntoRelationList, Projection};
use kova_kernel::types::{DecodeColumn, FromRow, Model, Update as NewUpdate};
use std::iter::once;
use std::marker::PhantomData;

pub struct Update<'c, 'a, T, E, R, S, X> {
    executor: &'c Executor<E>,
    statement: UpdateStatement<'a, Postgres, T>,
    _marker: PhantomData<(R, S, X)>,
}

pub struct Unset;
pub struct Assigned;

pub struct NoReturn;
pub struct Returning;

pub(crate) type InitUpdate<'c, 'a, T, E, X> = Update<'c, 'a, T, E, NoReturn, Unset, X>;

impl<'c, 'a, T, E, X> NewUpdate<'c, 'a, T, E, X> for Update<'c, 'a, T, E, NoReturn, Unset, X>
    where
        T: Model<Postgres, X> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: UpdateStatement::new(), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, S, X> Update<'c, 'a, T, E, NoReturn, S, X>
    where
        T: Model<Postgres, X> {
    pub fn set<A, M>(mut self, assignments: A) -> Update<'c, 'a, T, E, NoReturn, Assigned, X>
        where
            A: IntoArg<Assignment<'a, Postgres, X>, M> {
        self.statement.set(assignments.into_args_erased());
        Update { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, NoReturn, Assigned, X>
    where
        T: Model<Postgres, X> {
    pub fn returning<A, M>(mut self, cols: A) -> Update<'c, 'a, T, E, Returning, Assigned, X>
        where
            A: IntoArg<Projection<'a, Postgres, X>, M> {
        self.statement.returning(cols.into_args_erased());
        Update { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, R, S, X> Update<'c, 'a, T, E, R, S, X>
    where
        T: Model<Postgres, X> {
    pub fn from<REL: IntoRelationList>(mut self) -> Self {
        self.statement.from::<REL>();
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, NoReturn, Assigned, X>
    where
        T: Model<Postgres, X>,
        E: PgExecutor {
    pub async fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()?)).await
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, T, E, Returning, Assigned, X>
    where
        T: Model<Postgres, X>,
        E: PgExecutor {
    pub async fn execute_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<Postgres, X> {
        fetch_rows(self.executor, once(self.statement.build()?), |row| U::from_row(&row)).await
    }

    pub async fn execute_in<U>(self) -> Result<Vec<U>>
        where
            U: DecodeColumn<Postgres, X> {
        fetch_rows(self.executor, once(self.statement.build()?), |row| U::extract(&row)).await
    }
}