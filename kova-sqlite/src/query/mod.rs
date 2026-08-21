mod select;

use crate::query::select::Select;
use crate::types::Sqlite;
use kova_core::database::Executor;
use kova_core::types::{Model, Query as CoreQuery, Select as CoreSelect};

pub struct Query;

impl<'a> CoreQuery<'a, Sqlite> for Query {
    type Select<'c, T: Model<DB = Sqlite>, E> = Select<'c, 'a, T, E, Sqlite>
    where
        'a: 'c, E: 'c;

}

impl<'c, 'a, T, E> CoreSelect<'c, 'a, T, E, Sqlite> for Select<'c, 'a, T, E, Sqlite>
where
    T: Model<DB = Sqlite>,
{
    fn new(executor: &'c Executor<E>) -> Self {
        Select::new(executor)
    }
}

