
pub use kova_derive::{FromRow, Model};
pub use kova_core::{Error, Result};
// Client, Transaction

pub mod types {
    pub use kova_core::types::{Database, FromRow, Model, FromValue, ToValue, Extension, ColumnMeta};

    #[cfg(feature = "pg")]
    pub use kova_pg::types::{Postgres as Engine};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::types::{Sqlite as Engine};

    #[cfg(feature = "json")]
    pub use kova_core::Json;
}

pub mod dsl {
    // pub use crate::ops::dsl::assignment::Assignment;
    // pub use crate::ops::dsl::column::Column;
    // pub use crate::ops::dsl::condition::Condition;
    // pub use crate::ops::dsl::order::{OrderBy, OrderDir};
}

pub mod query {
    // #[cfg(feature = "pg")]
    // pub use kova_pg::query::{Insert, InsertUpdate, Select};
    //
    // #[cfg(feature = "sqlite")]
    // pub use kova_sqlite::query::{Insert, InsertUpdate, Select};

    // pub use crate::ops::delete::Delete;
    // pub use crate::ops::insert::{Insert, InsertUpdate};
    // pub use crate::ops::raw::Raw;
    // pub use crate::ops::select::Select;
    // pub use crate::ops::update::Update;
}


#[cfg(all(feature = "pg", feature = "sqlite"))]
compile_error!("Features 'pg' and 'sqlite' are mutually exclusive in Kova ORM.");



#[cfg(test)]
mod tests {
    use crate::types::{Engine, FromRow, Model};
    use kova_core::types::Database;

    use super::*;

    struct User {
        id: i32,
        name: String,
    }

    impl FromRow for User {
        type DB = Engine;

        fn from_row(row: &<Self::DB as Database>::Row<'_>) -> Result<Self, Error> {
            todo!()
        }

        fn from_row_as_values<'a>(row: &'a <Self::DB as Database>::Row<'_>) -> Result<Vec<<Self::DB as Database>::Value<'a>>, Error> {
            todo!()
        }
    }

    impl Model for User {
        const TABLE_NAME: &'static str = "";
        const COLUMNS_META: &'static [kova_core::types::ColumnMeta] = &[];

        fn to_values<'a>(&'a self) -> Result<Vec<(&'static str, <Self::DB as Database>::Value<'a>)>, Error> {
            todo!()
        }
    }


}