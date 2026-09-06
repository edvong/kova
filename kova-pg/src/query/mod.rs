mod insert;
mod select;
mod update;
mod delete;
mod statement;
mod exec;

use crate::query::insert::NoConflict;
use crate::types::Postgres;
use kova_core::types::{Database, Model, Query as CoreQuery};

pub use crate::query::select::Select;
pub use crate::query::insert::Insert;
pub use crate::query::update::Update;
pub use crate::query::delete::Delete;
use crate::query::statement::Statement;

pub struct NoReturn;
pub struct Returning;

pub struct Query;

impl<'a> CoreQuery<'a, Postgres> for Query {
    type Select<'c, T: Model<DB = Postgres>, E, X> = Select<'c, 'a, Postgres, T, E, X>
    where
        'a: 'c,
        E: 'c;

    type Insert<'c, T, E, X> = Insert<'c, 'a, Postgres, T, E, X, NoReturn, NoConflict>
    where
        T: Model<DB = Postgres> + 'a,
        'a: 'c,
        E: 'c;

    type Update<'c, T: Model<DB = Postgres>, E, X> = Update<'c, 'a, Postgres, T, E, X, NoReturn>
    where
        'a: 'c,
        E: 'c;

    type Delete<'c, T: Model<DB = Postgres>, E, X> = Delete<'c, 'a, Postgres, T, E, X, NoReturn>
    where
        'a: 'c,
        E: 'c;
    
    type Statement<'c, E, X> = Statement<'c, 'a, Postgres, E, X>
    where
        'a: 'c,
        E: 'c;
}
