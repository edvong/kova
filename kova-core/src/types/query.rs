use crate::database::Executor;
use crate::types::{Database, Model};

pub trait Query<'a, DB: Database> {
    type Select<'c, T: Model<DB = DB>, E>: Select<'c, 'a, T, E, DB> where 'a: 'c, E: 'c;

}

pub trait Select<'c, 'a, T, E, DB: Database> {
    fn new(executor: &'c Executor<E>) -> Self;
}

