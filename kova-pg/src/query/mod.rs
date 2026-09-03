mod insert;
mod select;
mod update;
mod delete;
mod statement;
mod exec;

use crate::query::insert::NoConflict;
use crate::types::Postgres;
use kova_core::types::{Model, Query as CoreQuery};

pub use crate::query::select::Select;
pub use crate::query::insert::Insert;
pub use crate::query::update::Update;
pub use crate::query::delete::Delete;
use crate::query::statement::Statement;

pub struct NoReturn;
pub struct Returning;

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

    type Update<'c, T: Model<DB = Postgres>, E> = Update<'c, 'a, T, E, Postgres, NoReturn>
    where
        'a: 'c,
        E: 'c;

    type Delete<'c, T: Model<DB = Postgres>, E> = Delete<'c, 'a, T, E, Postgres, NoReturn>
    where
        'a: 'c,
        E: 'c;
    type Statement<'c, E> = Statement<'c, 'a, E>
    where
        'a: 'c,
        E: 'c;

}
