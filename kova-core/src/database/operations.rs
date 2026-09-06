use crate::database::Executor;
use crate::dsl::IntoRows;
use crate::types::{Database, Delete, Insert, Model, Query, Select, Statement, Update};
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct Operations<'a, DB, E, X = ()> {
    pub(crate) executor: Executor<E>,
    pub(crate) _marker: PhantomData<(&'a X, DB)>,
}

impl<'a, DB, E, X> Operations<'a, DB, E, X> {
    pub fn new(e: Executor<E>) -> Self {
        Self {
            executor: e,
            _marker: PhantomData,
        }
    }
}

impl<'c, DB: Database, E, X> Operations<'c, DB, E, X> {
    pub fn select<'a, T>(&'c self) -> <DB::Query<'a> as Query<'a, DB>>::Select<'c, T, E, X>
    where
        T: Model<DB = DB>,
        'a: 'c,
        E: 'c,
    {
        <DB::Query<'a> as Query<'a, DB>>::Select::new(&self.executor)
    }

    pub fn insert<'a, T, D, M>(
        &'c self,
        data: D,
    ) -> <DB::Query<'a> as Query<'a, DB>>::Insert<'c, T, E, X>
    where
        T: Model<DB = DB>,
        D: IntoRows<'a, T, M>,
        'a: 'c,
        E: 'c,
    {
        <DB::Query<'a> as Query<'a, DB>>::Insert::new(&self.executor, data)
    }

    pub fn update<'a, T>(&'c self) -> <DB::Query<'a> as Query<'a, DB>>::Update<'c, T, E, X>
    where
        T: Model<DB = DB>,
        'a: 'c,
        E: 'c,
    {
        <DB::Query<'a> as Query<'a, DB>>::Update::new(&self.executor)
    }

    pub fn delete<'a, T>(&'c self) -> <DB::Query<'a> as Query<'a, DB>>::Delete<'c, T, E, X>
    where
        T: Model<DB = DB>,
        'a: 'c,
        E: 'c,
    {
        <DB::Query<'a> as Query<'a, DB>>::Delete::new(&self.executor)
    }

    pub fn statement<'a, S>(
        &'c self,
        sql: S,
    ) -> <DB::Query<'a> as Query<'a, DB>>::Statement<'c, E, X>
    where
        S: Into<Cow<'a, str>>,
        'a: 'c,
        E: 'c,
    {
        <DB::Query<'a> as Query<'a, DB>>::Statement::new(&self.executor, sql)
    }
}
