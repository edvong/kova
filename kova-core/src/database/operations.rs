use std::marker::PhantomData;
use crate::database::Executor;
use crate::dsl::IntoRows;
use crate::types::{Database, Insert, Model, Query, Select};

pub struct Operations<'a, E, DB> {
    pub(crate) executor: Executor<E>,
    pub(crate) _marker: PhantomData<&'a DB>
}

impl<'a, E, DB> Operations<'a, E, DB> {
    pub fn new(e: Executor<E>) -> Self {
        Self { executor: e, _marker: PhantomData }
    }
}

impl<'c, E, DB: Database> Operations<'c, E, DB> {
    pub fn select<'a, T>(&'c self) -> <DB::Query<'a> as Query<'a, DB>>::Select<'c, T, E>
    where
        T: Model<DB = DB>,
        'a: 'c,
        E: 'c,
    {
        <DB::Query<'a> as Query<'a, DB>>::Select::new(&self.executor)
    }

    pub fn insert<'a, T, D, M>(&'c self, data: D) -> <DB::Query<'a> as Query<'a, DB>>::Insert<'c, T, E>
    where
        T: Model<DB = DB>,
        D: IntoRows<'a, T, M>,
        'a: 'c,
        E: 'c,
    {
        <DB::Query<'a> as Query<'a, DB>>::Insert::<'c, T, E>::new(&self.executor, data)
    }

    // pub fn update<T>(&self) -> Update<'a, DB, T> {
    //     Update::create(&self.executor)
    // }
    //
    // pub fn delete<T>(&self) -> Delete<'a, DB, T> {
    //     Delete::create(&self.executor)
    // }
    //
    // pub fn statement<T>(&self) -> Statement<'a, DB, T> {
    //     Statement::create(&self.executor)
    // }
}
