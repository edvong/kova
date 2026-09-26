mod delete;
mod exec;
mod insert;
mod select;
mod statement;
mod update;

use crate::query::insert::NoConflict;
use crate::types::Postgres;
use kova_core::types::{Model, Query as CoreQuery};

pub use crate::query::delete::Delete;
pub use crate::query::insert::Insert;
use crate::query::select::{JustAddFrom, OuterQuery};
use crate::query::statement::Statement;
pub use crate::query::update::Update;
pub use crate::query::select::{Select};
pub(crate) use crate::query::select::Dispatch;

pub struct NoReturn;
pub struct Returning;

pub struct Query;

#[rustfmt::skip]
impl<'a> CoreQuery<'a, Postgres> for Query {
    type Select<'c, T: Model<DB = Postgres>, E, X> = Select<'a, T, OuterQuery<'c, E>, JustAddFrom, X>
        where
            'a: 'c,
            E: 'c;

    type Insert<'c, T, E, X> = Insert<'c, 'a, T, E, X, NoReturn, NoConflict>
        where
            T: Model<DB = Postgres> + 'a,
            'a: 'c,
            E: 'c;

    type Update<'c, T: Model<DB = Postgres>, E, X> = Update<'c, 'a, T, E, X, NoReturn>
        where
            'a: 'c,
            E: 'c;

    type Delete<'c, T: Model<DB = Postgres>, E, X> = Delete<'c, 'a, T, E, X, NoReturn>
        where
            'a: 'c,
            E: 'c;

    type Statement<'c, E, X> = Statement<'c, 'a, E, X>
        where
            'a: 'c,
            E: 'c;
}
