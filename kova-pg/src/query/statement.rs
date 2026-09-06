use crate::query::exec::{execute_affected, fetch_rows};
use crate::types::Postgres;
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Bind, IntoArgs};
use kova_core::statement::RawStatement;
use kova_core::types::{Database, ExecutableAsync, FromColumn, FromRow, Statement as CoreStatement};
use std::borrow::Cow;
use std::iter::once;
use std::marker::PhantomData;

pub struct Statement<'c, 'a, DB: Database, E, X> {
    executor: &'c Executor<E>,
    statement: RawStatement<'a, DB>,
    _marker: PhantomData<X>,
}

impl<'c, 'a, E, X> CoreStatement<'c, 'a, Postgres, E, X> for Statement<'c, 'a, Postgres, E, X> {
    fn new<S: Into<Cow<'a, str>>>(executor: &'c Executor<E>, sql: S) -> Self {
        Self {
            executor,
            statement: RawStatement::new(sql),
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, E, X> Statement<'c, 'a, Postgres, E, X> {
    pub fn bind<A>(&mut self, args: A)
    where
        A: IntoArgs<Bind<'a, Postgres>>,
    {
        self.statement.bind(args.into_args());
    }
}

impl<'c, 'a, E, X> Statement<'c, 'a, Postgres, E, X>
where
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn execute(self) -> Result<u64> {
        execute_affected(self.executor, once(self.statement.build()?)).await
    }
    
    pub async fn load_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Postgres>,
    {
        fetch_rows(self.executor, once(self.statement.build()?), |row| U::from_row(&row)).await
    }

    pub async fn load_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: FromColumn<'r, Postgres, X>,
    {
        fetch_rows(self.executor, once(self.statement.build()?), |row| U::extract(&row)).await
    }
}
