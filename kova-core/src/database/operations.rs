use crate::database::Executor;
use crate::types::{ExecutableAsync, ExecutableSync};

pub struct Operations<E> {
    pub(crate) executor: Executor<E>,
}

impl<E> Operations<E> {
    pub fn new(e: Executor<E>) -> Self {
        Self { executor: e }
    }
}

impl<E: ExecutableSync> Operations<E> {
    // pub fn select<T>(&self) -> Select<'a, DB, T> {
    //     Select::create(&self.executor)
    // }
    //
    // pub fn insert<T>(&self, T/Iterator<Item=T>) -> Insert<'a, DB, T> {
    //     Insert::create(&self.executor)
    // }
    //
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

impl<E: ExecutableAsync> Operations<E> {

}