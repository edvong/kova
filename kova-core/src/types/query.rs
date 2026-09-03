use std::borrow::Cow;
use crate::database::Executor;
use crate::dsl::IntoRows;
use crate::types::{Database, Model};

pub trait Query<'a, DB: Database> {
    type Select<'c, T: Model<DB = DB>, E>: Select<'c, 'a, T, E, DB>
    where
        'a: 'c,
        E: 'c;

    type Insert<'c, T, E>: Insert<'c, 'a, T, E, DB>
    where
        T: Model<DB = DB> + 'a,
        'a: 'c,
        E: 'c;

    type Update<'c, T: Model<DB = DB>, E>: Update<'c, 'a, T, E, DB>
    where
        'a: 'c,
        E: 'c;

    type Delete<'c, T: Model<DB = DB>, E>: Delete<'c, 'a, T, E, DB>
    where
        'a: 'c,
        E: 'c;

    type Statement<'c, E>: Statement<'c, 'a, E, DB>
    where
        'a: 'c,
        E: 'c;
}

pub trait Select<'c, 'a, T, E, DB: Database> {
    fn new(executor: &'c Executor<E>) -> Self;
}

pub trait Insert<'c, 'a, T, E, DB: Database> {
    fn new<D, M>(executor: &'c Executor<E>, data: D) -> Self
    where
        D: IntoRows<'a, T, M>;
}

pub trait Update<'c, 'a, T, E, DB: Database> {
    fn new(executor: &'c Executor<E>) -> Self;
}

pub trait Delete<'c, 'a, T, E, DB: Database> {
    fn new(executor: &'c Executor<E>) -> Self;
}

pub trait Statement<'c, 'a, E, DB: Database> {
    fn new<S: Into<Cow<'a, str>>>(executor: &'c Executor<E>, sql: S) -> Self;
}
