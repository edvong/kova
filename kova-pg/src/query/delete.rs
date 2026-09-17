use crate::query::exec::{execute_affected, fetch_rows};
use crate::query::{NoReturn, Returning};
use crate::types::Postgres;
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Condition, EraseX, IntoArgs, Projection};
use kova_core::statement::DeleteStatement;
use kova_core::types::{
    Database, Delete as CoreDelete, ExecutableAsync, FromColumn, FromRow, Model,
};
use std::iter::once;
use std::marker::PhantomData;

pub struct Delete<'c, 'a, DB: Database, T, E, X, R = NoReturn> {
    executor: &'c Executor<E>,
    statement: DeleteStatement<'a, DB, T>,
    _marker: PhantomData<(X, R)>,
}

impl<'c, 'a, T, E, X> CoreDelete<'c, 'a, Postgres, T, E, X>
    for Delete<'c, 'a, Postgres, T, E, X, NoReturn>
    where
        T: Model<DB = Postgres> {
    fn new(executor: &'c Executor<E>) -> Self {
        Self { executor, statement: DeleteStatement::new(), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X> Delete<'c, 'a, Postgres, T, E, X, NoReturn>
    where
        T: Model<DB = Postgres> {
    pub fn returning<P>(mut self, cols: P) -> Delete<'c, 'a, Postgres, T, E, X, Returning>
        where
            P: IntoArgs<Projection<'a, Postgres, X>> {
        self.statement.returning(cols.into_mapped_args(|p| p.erase_x()));
        Delete { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X, R> Delete<'c, 'a, Postgres, T, E, X, R>
    where
        T: Model<DB = Postgres> {
    pub fn filter(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }
}

impl<'c, 'a, T, E, X> Delete<'c, 'a, Postgres, T, E, X, NoReturn>
    where
        T: Model<DB = Postgres>,
        E: ExecutableAsync,
        E::Native: GenericClient + Sync {
    pub async fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build())).await
    }
}

impl<'c, 'a, T, E, X> Delete<'c, 'a, Postgres, T, E, X, Returning>
    where
        T: Model<DB = Postgres>,
        E: ExecutableAsync,
        E::Native: GenericClient + Sync {
    pub async fn execute_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<DB = Postgres> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::from_row(&row)).await
    }

    pub async fn execute_in<U>(self) -> Result<Vec<U>>
        where
            for<'r> U: FromColumn<'r, Postgres, X> {
        fetch_rows(self.executor, once(self.statement.build()), |row| U::extract(&row)).await
    }
}
