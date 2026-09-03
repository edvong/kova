use crate::dsl::{ConflictTarget, OnConflict};
use crate::query::exec::{execute_affected, fetch_rows};
use crate::query::{NoReturn, Returning};
use crate::types::Postgres;
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Assignment, IntoArgs, IntoRows, Projection};
use kova_core::statement::InsertStatement;
use kova_core::types::{
    Database, ExecutableAsync, FromColumn, FromRow, Insert as CoreInsert, Model,
};
use std::marker::PhantomData;

pub struct NoConflict;
pub struct Conflicted;

pub struct Insert<'c, 'a, T, E, DB: Database, R = NoReturn, C = NoConflict> {
    executor: &'c Executor<E>,
    statement: InsertStatement<'a, T, DB>,
    _marker: PhantomData<(R, C)>,
}

impl<'c, 'a, T, E> CoreInsert<'c, 'a, T, E, Postgres>
for Insert<'c, 'a, T, E, Postgres, NoReturn, NoConflict>
where
    T: Model<DB = Postgres>,
{
    fn new<D, M>(executor: &'c Executor<E>, data: D) -> Self
    where
        D: IntoRows<'a, T, M>,
    {
        Self {
            executor,
            statement: InsertStatement::new(data.into_rows()),
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, C> Insert<'c, 'a, T, E, Postgres, NoReturn, C>
where
    T: Model<DB = Postgres>,
{
    pub fn returning<P>(mut self, cols: P) -> Insert<'c, 'a, T, E, Postgres, Returning, C>
    where
        P: IntoArgs<Projection<'a, Postgres>>,
    {
        self.statement.retuning(cols);
        Insert {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, R> Insert<'c, 'a, T, E, Postgres, R, NoConflict>
where
    T: Model<DB = Postgres>,
{
    pub fn on_conflict<A>(mut self, targets: A) -> Insert<'c, 'a, T, E, Postgres, R, Conflicted>
    where
        A: IntoArgs<ConflictTarget<'a>>,
    {
        let c = self.statement.on_conflict_ext_mut_or(|| OnConflict::new());

        let mut target_ast = Vec::new();
        targets.append_args(&mut target_ast);
        c.conflict(target_ast);

        Insert {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, R> Insert<'c, 'a, T, E, Postgres, R, Conflicted>
where
    T: Model<DB = Postgres>,
{
    pub fn ignore(mut self) -> Self {
        let c = self.statement.on_conflict_ext_mut_or(|| OnConflict::new());
        c.ignore();

        self
    }

    pub fn update<A>(mut self, assignments: A) -> Self
    where
        A: IntoArgs<Assignment<'a, Postgres>>,
    {
        let c = self.statement.on_conflict_ext_mut_or(|| OnConflict::new());
        let mut vec = Vec::new();
        assignments.append_args(&mut vec);
        c.update(vec);

        self
    }
}

impl<'c, 'a, T, E, C> Insert<'c, 'a, T, E, Postgres, NoReturn, C>
where
    T: Model<DB = Postgres>,
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn execute(self) -> Result<u64> {
        execute_affected(self.executor, self.statement.build()).await
    }
}

impl<'c, 'a, T, E, C> Insert<'c, 'a, T, E, Postgres, Returning, C>
where
    T: Model<DB = Postgres>,
    E: ExecutableAsync,
    E::Native: GenericClient + Sync,
{
    pub async fn execute_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Postgres>,
    {
        fetch_rows(self.executor, self.statement.build(), |row| U::from_row(&row)).await
    }

    pub async fn execute_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: FromColumn<'r, Postgres>,
    {
        fetch_rows(self.executor, self.statement.build(), |row| U::extract(&row)).await
    }
}
