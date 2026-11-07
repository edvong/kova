mod delete;
#[rustfmt::skip]
mod exec;
mod insert;
mod select;
mod statement;
mod update;

use kova_kernel::types::Model;
use crate::types::Postgres;
use crate::query::delete::InitDelete;
use crate::query::insert::InitInsert;
use crate::query::select::InitSelect;
use crate::query::update::InitUpdate;
pub(crate) use crate::query::select::InnerSelect;

pub use {insert::Insert, select::Select, update::Update, delete::Delete, statement::Statement};


pub struct Query;

#[rustfmt::skip]
impl<'a> kova_kernel::types::Query<'a, Postgres> for Query {
    type Select<'c, T: Model<Postgres, X>, E, X> = InitSelect<'c, 'a, T, E, X>
        where
            'a: 'c,
            E: 'c;

    type Insert<'c, T, E, X> = InitInsert<'c, 'a, T, E, X>
        where
            T: Model<Postgres, X> + 'a,
            'a: 'c,
            E: 'c;

    type Update<'c, T: Model<Postgres, X>, E, X> = InitUpdate<'c, 'a, T, E, X>
        where
            'a: 'c,
            E: 'c;

    type Delete<'c, T: Model<Postgres, X>, E, X> = InitDelete<'c, 'a, T, E, X>
        where
            'a: 'c,
            E: 'c;

    type Statement<'c, E, X> = Statement<'c, 'a, E, X>
        where
            'a: 'c,
            E: 'c;
}
