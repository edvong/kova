mod select;

use crate::types::Postgres;
use kova_core::types::{Model, Query as CoreQuery};

pub use crate::query::select::Select;


pub struct Query;

impl<'a> CoreQuery<'a, Postgres> for Query {
    type Select<'c, T: Model<DB = Postgres>, E>
        = Select<'c, 'a, T, E, Postgres>
    where
        'a: 'c,
        E: 'c;
}
