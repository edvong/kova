mod select;

use crate::query::select::Select;
use crate::types::Sqlite;
use kova_core::types::{Model, Query as CoreQuery};

pub struct Query;

impl<'a> CoreQuery<'a, Sqlite> for Query {
    type Select<'c, T: Model<DB = Sqlite>, E> = Select<'c, 'a, T, E, Sqlite>
    where
        'a: 'c, E: 'c;

}


