use crate::database::Executor;
use crate::dsl::IntoInsertSource;
use crate::types::{Database, Delete, Insert, Model, Query, Select, Statement, Update};
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct Operations<DB, E, X> {
    pub(crate) executor: Executor<E>,
    pub(crate) _marker: PhantomData<(DB, X)>,
}

impl<DB, E: Clone, X> Clone for Operations<DB, E, X> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { executor: self.executor.clone(), _marker: PhantomData }
    }
}

impl<DB, E, X> Operations<DB, E, X> {
    pub(crate) fn new(e: Executor<E>) -> Self {
        Self { executor: e, _marker: PhantomData }
    }

    pub(crate) fn codec<Codec>(self) -> Operations<DB, E, Codec> {
        Operations { executor: self.executor, _marker: PhantomData }
    }
}

impl<'c, DB: Database, E, X> Operations<DB, E, X> {
    pub fn select<'a, T>(&'c self) -> <DB::Query as Query<'a, DB>>::Select<'c, T, E, X>
        where
            T: Model<DB, X>,
            'a: 'c,
            E: 'c {
        <DB::Query as Query<'a, DB>>::Select::new(&self.executor)
    }

    pub fn insert<'a, T>(&'c self, data: impl IntoInsertSource<'a, DB, T, X>) 
        -> <DB::Query as Query<'a, DB>>::Insert<'c, T, E, X>
        where
            T: Model<DB, X>,
            'a: 'c,
            E: 'c {
        <DB::Query as Query<'a, DB>>::Insert::new(&self.executor, data)
    }

    pub fn update<'a, T>(&'c self) -> <DB::Query as Query<'a, DB>>::Update<'c, T, E, X>
        where
            T: Model<DB, X>,
            'a: 'c,
            E: 'c {
        <DB::Query as Query<'a, DB>>::Update::new(&self.executor)
    }

    pub fn delete<'a, T>(&'c self) -> <DB::Query as Query<'a, DB>>::Delete<'c, T, E, X>
        where
            T: Model<DB, X>,
            'a: 'c,
            E: 'c {
        <DB::Query as Query<'a, DB>>::Delete::new(&self.executor)
    }

    pub fn statement<'a, S>(&'c self, sql: S) -> <DB::Query as Query<'a, DB>>::Statement<'c, E, X>
        where
            S: Into<Cow<'a, str>>,
            'a: 'c,
            E: 'c {
        <DB::Query as Query<'a, DB>>::Statement::new(&self.executor, sql)
    }
}
