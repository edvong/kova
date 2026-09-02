use crate::dsl::{ConflictTarget, OnConflict};
use crate::types::{Row, Sqlite};
use kova_core::database::Executor;
use kova_core::dsl::{Assignment, IntoArgs, IntoRows, Projection};
use kova_core::statement::InsertStatement;
use kova_core::types::{
    Database, ExecutableSync, FromColumn, FromRow, Insert as CoreInsert, Model,
};
use kova_core::{Error, Result};
use std::marker::PhantomData;

pub struct NoReturn;
pub struct Returning;

pub struct NoConflict;
pub struct Conflicted;

pub struct Insert<'c, 'a, T, E, DB: Database, R = NoReturn, C = NoConflict> {
    executor: &'c Executor<E>,
    statement: InsertStatement<'a, T, DB>,
    _marker: PhantomData<(R, C)>,
}

impl<'c, 'a, T, E, C> Insert<'c, 'a, T, E, Sqlite, NoReturn, C>
where
    T: Model<DB = Sqlite>,
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

    pub fn returning<P>(mut self, cols: P) -> Insert<'c, 'a, T, E, Sqlite, Returning, C>
    where
        P: IntoArgs<Projection<'a, Sqlite>>,
    {
        self.statement.retuning(cols);
        Insert {
            executor: self.executor,
            statement: self.statement,
            _marker: PhantomData,
        }
    }
}

impl<'c, 'a, T, E, R> Insert<'c, 'a, T, E, Sqlite, R, NoConflict>
where
    T: Model<DB = Sqlite>,
{
    pub fn on_conflict<A>(mut self, targets: A) -> Insert<'c, 'a, T, E, Sqlite, R, Conflicted>
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

impl<'c, 'a, T, E, R> Insert<'c, 'a, T, E, Sqlite, R, Conflicted>
where
    T: Model<DB = Sqlite>,
{
    pub fn ignore(mut self) -> Self {
        let c = self.statement.on_conflict_ext_mut_or(|| OnConflict::new());
        c.ignore();

        self
    }

    pub fn update<A>(mut self, assignments: A) -> Self
    where
        A: IntoArgs<Assignment<'a, Sqlite>>,
    {
        let c = self.statement.on_conflict_ext_mut_or(|| OnConflict::new());
        let mut vec = Vec::new();
        assignments.append_args(&mut vec);
        c.update(vec);

        self
    }
}

impl<'c, 'a, T, E, C> Insert<'c, 'a, T, E, Sqlite, NoReturn, C>
where
    T: Model<DB = Sqlite>,
    E: ExecutableSync<Native = rusqlite::Connection>,
{
    pub fn execute(self) -> Result<u64> {
        let data = self.statement.build();

        self.executor.with_conn_sync(|native| {
            let mut stmt = native
                .prepare(&data.sql)
                .map_err(|e| Error::Database(Box::new(e)))?;

            let affected_rows = stmt
                .execute(rusqlite::params_from_iter(data.params))
                .map_err(|e| Error::Database(Box::new(e)))?;

            Ok(affected_rows as u64)
        })
    }
}

impl<'c, 'a, T, E, C> Insert<'c, 'a, T, E, Sqlite, Returning, C>
where
    T: Model<DB = Sqlite>,
    E: ExecutableSync<Native = rusqlite::Connection>,
{
    pub fn execute_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Sqlite>,
    {
        self.process_rows(|row| U::from_row(&row))
    }

    pub fn execute_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: FromColumn<'r, Sqlite>,
    {
        self.process_rows(|row| U::extract(&row))
    }

    fn process_rows<U, F>(self, mut f: F) -> Result<Vec<U>>
    where
        F: FnMut(Row<'_>) -> Result<U>,
    {
        let data = self.statement.build();
        self.executor.with_conn_sync(|native| {
            let mut stmt = native
                .prepare(&data.sql)
                .map_err(|e| Error::Database(Box::new(e)))?;

            let mut rows = stmt
                .query(rusqlite::params_from_iter(data.params))
                .map_err(|e| Error::Database(Box::new(e)))?;

            let mut result = Vec::new();
            while let Some(raw_row) = rows.next().map_err(|e| Error::Database(Box::new(e)))? {
                let row = Row::new(raw_row);
                result.push(f(row)?);
            }
            Ok(result)
        })
    }
}

impl<'c, 'a, T, E> CoreInsert<'c, 'a, T, E, Sqlite>
    for Insert<'c, 'a, T, E, Sqlite, NoReturn, NoConflict>
where
    T: Model<DB = Sqlite>,
{
    fn new<D, M>(executor: &'c Executor<E>, data: D) -> Self
    where
        D: IntoRows<'a, T, M>,
    {
        Self::new(executor, data)
    }
}
