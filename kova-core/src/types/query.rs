use std::borrow::Cow;
use crate::database::Executor;
use crate::dsl::IntoRows;
use crate::types::{Database, Model};

pub trait Query<'a, DB: Database> {
    type Select<'c, T: Model<DB = DB>, E, X>: Select<'c, 'a, DB, T, E, X>
    where
        'a: 'c,
        E: 'c;

    type Insert<'c, T, E, X>: Insert<'c, 'a, DB, T, E, X>
    where
        T: Model<DB = DB> + 'a,
        'a: 'c,
        E: 'c;

    type Update<'c, T: Model<DB = DB>, E, X>: Update<'c, 'a, DB, T, E, X>
    where
        'a: 'c,
        E: 'c;

    type Delete<'c, T: Model<DB = DB>, E, X>: Delete<'c, 'a, DB, T, E, X>
    where
        'a: 'c,
        E: 'c;

    type Statement<'c, E, X>: Statement<'c, 'a, DB, E, X>
    where
        'a: 'c,
        E: 'c;
}

pub trait Select<'c, 'a, DB: Database, T, E, X> {
    fn new(executor: &'c Executor<E>) -> Self;
}

pub trait Insert<'c, 'a, DB: Database, T, E, X> {
    fn new<D, M>(executor: &'c Executor<E>, data: D) -> Self
    where
        D: IntoRows<'a, T, M>;
}

pub trait Update<'c, 'a, DB: Database, T, E, X> {
    fn new(executor: &'c Executor<E>) -> Self;
}

pub trait Delete<'c, 'a, DB: Database, T, E, X> {
    fn new(executor: &'c Executor<E>) -> Self;
}

pub trait Statement<'c, 'a, DB: Database, E, X> {
    fn new<S: Into<Cow<'a, str>>>(executor: &'c Executor<E>, sql: S) -> Self;
}
