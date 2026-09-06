use crate::query::exec::{execute_affected, fetch_rows};
use crate::query::{NoReturn, Returning};
use crate::types::Postgres;
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Assignment, Condition, EraseX, IntoArgs, Projection};
use kova_core::statement::UpdateStatement;
use kova_core::types::{
    Database, ExecutableAsync, FromColumn, FromRow, Model, Update as CoreUpdate,
};
use std::iter::once;
use std::marker::PhantomData;

pub struct Update<'c, 'a, DB: Database, T, E, X, R = NoReturn> {
    executor: &'c Executor<E>,
    statement: UpdateStatement<'a, DB, T>,
    _marker: PhantomData<(R, X)>,
}

impl<'c, 'a, T, E, X> CoreUpdate<'c, 'a, Postgres, T, E, X>
    for Update<'c, 'a, Postgres, T, E, X, NoReturn>
where
    T: Model<DB = Postgres>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Self {
            executor,
            statement: UpdateStatement::new(),
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, Postgres, T, E, X, NoReturn>
where
    T: Model<DB = Postgres>,
{
    pub fn set<A>(mut self, assignments: A) -> Self
    where
        A: IntoArgs<Assignment<'a, Postgres, X>>,
    {
        self.statement
            .set(assignments.into_mapped_args(|a| a.erase_x()));

        self
    }

    pub fn returning<P>(mut self, cols: P) -> Update<'c, 'a, Postgres, T, E, X, Returning>
    where
        P: IntoArgs<Projection<'a, Postgres, X>>,
    {
        self.statement.returning(cols.into_mapped_args(|p| p.erase_x()));
        Update {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, X, R> Update<'c, 'a, Postgres, T, E, X, R>
where
    T: Model<DB = Postgres>,
{
    pub fn filter(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, Postgres, T, E, X, NoReturn>
where
    T: Model<DB = Postgres>,
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build())).await
    }
}

impl<'c, 'a, T, E, X> Update<'c, 'a, Postgres, T, E, X, Returning>
where
    T: Model<DB = Postgres>,
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn execute_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Postgres>,
    {
        fetch_rows(self.executor, once(self.statement.build()), |row| {
            U::from_row(&row)
        })
        .await
    }

    pub async fn execute_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: FromColumn<'r, Postgres, X>,
    {
        fetch_rows(self.executor, once(self.statement.build()), |row| {
            U::extract(&row)
        })
        .await
    }
}
