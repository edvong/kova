mod select;
mod insert;

use crate::query::insert::{NoConflict, NoReturn};
use crate::types::Sqlite;
use kova_core::types::{Model, Query as CoreQuery};

pub use crate::query::select::Select;
pub use crate::query::insert::Insert;

pub struct Query;

impl<'a> CoreQuery<'a, Sqlite> for Query {
    type Select<'c, T: Model<DB = Sqlite>, E> = Select<'c, 'a, T, E, Sqlite>
    where
        'a: 'c,
        E: 'c;

    type Insert<'c, T, E> = Insert<'c, 'a, T, E, Sqlite, NoReturn, NoConflict>
    where
        T: Model<DB=Sqlite> + 'a,
        'a: 'c,
        E: 'c;
}


