use crate::query::select::Select;
use crate::types::Postgres;
use kova_core::database::Executor;
use kova_core::types::{Model, Query as CoreQuery, Select as CoreSelect};

mod select;

pub struct Query;

impl<'a> CoreQuery<'a, Postgres> for Query {
    type Select<'c, T: Model<DB = Postgres>, E> = Select<'c, 'a, T, E, Postgres>
    where
        'a: 'c, E: 'c;

}

impl<'c, 'a, T, E> CoreSelect<'c, 'a, T, E, Postgres> for Select<'c, 'a, T, E, Postgres>
where
    T: Model<DB = Postgres>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Select::new(executor)
    }
}

