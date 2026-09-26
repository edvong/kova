mod delete;
mod exec;
mod insert;
mod select;
mod statement;
mod update;

use crate::query::insert::NoConflict;
use crate::types::Sqlite;
use kova_core::types::{Model, Query as CoreQuery};

pub use crate::query::delete::Delete;
pub use crate::query::insert::Insert;
use crate::query::select::{JustAddFrom, OuterQuery};
pub use crate::query::select::Select;
use crate::query::statement::Statement;
pub use crate::query::update::Update;

pub struct NoReturn;
pub struct Returning;

pub struct Query;

#[rustfmt::skip]
impl<'a> CoreQuery<'a, Sqlite> for Query {
    type Select<'c, T: Model<DB = Sqlite>, E, X> = Select<'a, T, OuterQuery<'c, E>, JustAddFrom, X>
        where
            'a: 'c,
            E: 'c;

    type Insert<'c, T, E, X> = Insert<'c, 'a, T, E, X, NoReturn, NoConflict>
        where
            T: Model<DB = Sqlite> + 'a,
            'a: 'c,
            E: 'c;

    type Update<'c, T: Model<DB = Sqlite>, E, X> = Update<'c, 'a, T, E, X, NoReturn>
        where
            'a: 'c,
            E: 'c;

    type Delete<'c, T: Model<DB = Sqlite>, E, X> = Delete<'c, 'a, T, E, X, NoReturn>
        where
            'a: 'c,
            E: 'c;

    type Statement<'c, E, X> = Statement<'c, 'a, E, X>
        where
            'a: 'c,
            E: 'c;
}
