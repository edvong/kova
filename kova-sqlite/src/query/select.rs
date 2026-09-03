use crate::dsl::Pagination;
use crate::query::exec::fetch_rows;
use crate::types::Sqlite;
use kova_core::Result;
use kova_core::database::Executor;
use kova_core::dsl::{Condition, GroupBy, IntoArgs, Join, JoinType, Ordering, Projection};
use kova_core::statement::SelectStatement;
use kova_core::types::{
    Database, ExecutableSync, FromColumn, FromRow, Model, Select as CoreSelect,
};
use std::iter::once;

pub struct Select<'c, 'a, T, E, DB: Database> {
    pub(crate) executor: &'c Executor<E>,
    pub(crate) statement: SelectStatement<'a, T, DB>,
}

impl<'c, 'a, T, E> CoreSelect<'c, 'a, T, E, Sqlite> for Select<'c, 'a, T, E, Sqlite>
where
    T: Model<DB = Sqlite>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Self {
            executor,
            statement: SelectStatement::new(),
        }
    }
}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Sqlite>
where
    T: Model<DB = Sqlite>,
{
    pub fn columns<C>(mut self, cols: C) -> Self
    where
        C: IntoArgs<Projection<'a, Sqlite>>,
    {
        self.statement.columns(cols);
        self
    }

    pub fn inner_join<M>(mut self, cond: Condition<'a, Sqlite>) -> Self
    where
        M: Model<DB = Sqlite>,
    {
        let join = Join {
            typ: JoinType::Inner,
            target_table: T::TABLE_NAME,
            on: cond,
        };
        self.statement.join(join);
        self
    }

    pub fn left_join<M>(mut self, cond: Condition<'a, Sqlite>) -> Self
    where
        M: Model<DB = Sqlite>,
    {
        let join = Join {
            typ: JoinType::Left,
            target_table: T::TABLE_NAME,
            on: cond,
        };
        self.statement.join(join);
        self
    }

    pub fn filter(mut self, cond: Condition<'a, Sqlite>) -> Self {
        self.statement.r#where(cond);
        self
    }

    pub fn group_by<C>(mut self, cols: C) -> Self
    where
        C: IntoArgs<GroupBy<'a, Sqlite>>,
    {
        self.statement.group_by(cols);
        self
    }

    pub fn having(mut self, cond: Condition<'a, Sqlite>) -> Self {
        self.statement.having(cond);
        self
    }

    pub fn order_by<C>(mut self, cols: C) -> Self
    where
        C: IntoArgs<Ordering<'a, Sqlite>>,
    {
        self.statement.order_by(cols);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        let ext = self
            .statement
            .pagination_ext_mut_or_with(|| Pagination::default());
        ext.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        let ext = self
            .statement
            .pagination_ext_mut_or_with(|| Pagination::default());
        ext.offset = Some(offset);
        self
    }
}

impl<'c, 'a, T, E> Select<'c, 'a, T, E, Sqlite>
where
    T: Model<DB = Sqlite>,
    E: ExecutableSync<Native = rusqlite::Connection>,
{
    pub fn load(self) -> Result<Vec<T>> {
        self.load_as::<T>()
    }

    pub fn load_as<U>(self) -> Result<Vec<U>>
    where
        U: FromRow<DB = Sqlite>,
    {
        fetch_rows(self.executor, once(self.statement.build()), |row| {
            U::from_row(&row)
        })
    }

    pub fn load_in<U>(self) -> Result<Vec<U>>
    where
        for<'r> U: FromColumn<'r, Sqlite>,
    {
        fetch_rows(self.executor, once(self.statement.build()), |row| {
            U::extract(&row)
        })
    }
}
