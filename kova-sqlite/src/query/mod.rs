mod select;
mod insert;
mod update;
mod delete;
mod statement;
mod exec;

use crate::query::insert::NoConflict;
use crate::types::Sqlite;
use kova_core::types::{Model, Query as CoreQuery};

pub use crate::query::select::Select;
pub use crate::query::insert::Insert;
pub use crate::query::update::Update;
pub use crate::query::delete::Delete;
use crate::query::statement::Statement;

pub struct NoReturn;
pub struct Returning;

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
    type Update<'c, T: Model<DB = Sqlite>, E> = Update<'c, 'a, T, E, Sqlite, NoReturn>
    where
        'a: 'c,
        E: 'c;

    type Delete<'c, T: Model<DB = Sqlite>, E> = Delete<'c, 'a, T, E, Sqlite, NoReturn>
    where
        'a: 'c,
        E: 'c;

    type Statement<'c, E> = Statement<'c, 'a, E>
    where
        'a: 'c,
        E: 'c;
}
