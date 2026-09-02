mod insert;
mod select;

use crate::query::insert::{NoConflict, NoReturn};
use crate::types::Postgres;
use kova_core::types::{Model, Query as CoreQuery};

pub use crate::query::select::Select;
pub use crate::query::insert::Insert;

pub struct Query;

impl<'a> CoreQuery<'a, Postgres> for Query {
    type Select<'c, T: Model<DB = Postgres>, E> = Select<'c, 'a, T, E, Postgres>
    where
        'a: 'c,
        E: 'c;

    type Insert<'c, T, E> = Insert<'c, 'a, T, E, Postgres, NoReturn, NoConflict>
    where
        T: Model<DB = Postgres> + 'a,
        'a: 'c,
        E: 'c;
}
