use crate::dsl::ConflictTarget;
use crate::query::exec::{execute_affected, fetch_rows};
use crate::query::{NoReturn, Returning};
use crate::types::Postgres;
use deadpool_postgres::GenericClient;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::statement::InsertStatement;
use kova_core::dsl::{Assignment, Condition, EraseX, IntoArgs, IntoInsertSource, Projection};
use kova_core::types::{
    Database, ExecutableAsync, FromColumn, FromRow, Insert as CoreInsert, Model,
};
use std::marker::PhantomData;

pub struct NoConflict;
pub struct Conflicted;

pub struct Insert<'c, 'a, DB: Database, T, E, X, R = NoReturn, C = NoConflict> {
    executor: &'c Executor<E>,
    statement: InsertStatement<'a, DB, T>,
    _marker: PhantomData<(X, R, C)>,
}

impl<'c, 'a, T, E, X> CoreInsert<'c, 'a, Postgres, T, E, X>
    for Insert<'c, 'a, Postgres, T, E, X, NoReturn, NoConflict>
    where
        T: Model<DB = Postgres> {
    fn new<D>(executor: &'c Executor<E>, data: D) -> Self
        where
            D: IntoInsertSource<'a, T::DB, T, X> {
        Self { executor, statement: InsertStatement::new(data), _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X, C> Insert<'c, 'a, Postgres, T, E, X, NoReturn, C>
    where
        T: Model<DB = Postgres> {
    pub fn returning<A>(mut self, cols: A) -> Insert<'c, 'a, Postgres, T, E, X, Returning, C>
        where
            A: IntoArgs<Projection<'a, Postgres, X>> {
        self.statement.retuning(cols);
        Insert { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X, R> Insert<'c, 'a, Postgres, T, E, X, R, NoConflict>
    where
        T: Model<DB = Postgres> {
    pub fn on_conflict<A>(
        mut self, targets: A,
    ) -> Insert<'c, 'a, Postgres, T, E, X, R, Conflicted>
        where
            A: IntoArgs<ConflictTarget<'a>> {
        self.statement.conflict_mut().set_conflict(targets.into_args());
        Insert { executor: self.executor, statement: self.statement, _marker: PhantomData }
    }
}

impl<'c, 'a, T, E, X, R> Insert<'c, 'a, Postgres, T, E, X, R, Conflicted>
    where
        T: Model<DB = Postgres> {
    pub fn do_nothing(mut self) -> Self {
        self.statement.conflict_mut().set_ignore();
        self
    }

    pub fn update<A>(mut self, assignments: A) -> Self
        where
            A: IntoArgs<Assignment<'a, Postgres, X>> {
        self.statement.conflict_mut().set_assignment(assignments.into_mapped_args(|a| a.erase_x()));
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Postgres, X>) -> Self {
        self.statement.r#where(cond);
        self
    }
}

impl<'c, 'a, T, E, X, C> Insert<'c, 'a, Postgres, T, E, X, NoReturn, C>
    where
        T: Model<DB = Postgres>,
        E: ExecutableAsync,
        E::Native: GenericClient + Sync {
    pub async fn execute(self) -> Result<u64> {
        execute_affected(self.executor, self.statement.build()).await
    }
}

impl<'c, 'a, T, E, X, C> Insert<'c, 'a, Postgres, T, E, X, Returning, C>
    where
        T: Model<DB = Postgres>,
        E: ExecutableAsync,
        E::Native: GenericClient + Sync {
    pub async fn execute_as<U>(self) -> Result<Vec<U>>
        where
            U: FromRow<DB = Postgres> {
        fetch_rows(self.executor, self.statement.build(), |row| U::from_row(&row)).await
    }

    pub async fn execute_in<U>(self) -> Result<Vec<U>>
        where
            for<'r> U: FromColumn<'r, Postgres, X> {
        fetch_rows(self.executor, self.statement.build(), |row| U::extract(&row)).await
    }
}
