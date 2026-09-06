mod macros;

pub use kova_core::database::{Client, Transaction};
pub use kova_core::{Error, Result};
pub use kova_derive::{FromRow, Model};

pub mod types {
    pub use kova_core::types::{
        ColumnMeta, Database, Decode, DecodeExt, Encode, FromRow, FromValue, IntoValue, Model, Row,
    };

    #[cfg(feature = "pg")]
    pub use kova_pg::types::Postgres as Engine;

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::types::Sqlite as Engine;
}

pub mod dsl {
    pub use kova_core::dsl::{Column, Condition};

    #[cfg(feature = "pg")]
    pub use kova_pg::dsl::{LockMode, WaitPolicy};

    #[doc(hidden)]
    #[cfg(feature = "pg")]
    pub use kova_pg::dsl::Column as ExtColumn;

    #[doc(hidden)]
    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::dsl::Column as ExtColumn;

    pub mod func {
        pub use kova_core::dsl::func::{
            Abs,
            Avg,
            Count,
            // Function, Quantifier,
            CountAll,
            Length,
            Lower,
            Max,
            Min,
            Sum,
            Upper,
        };

        #[cfg(feature = "pg")]
        pub use kova_pg::dsl::func::ArrayAgg;
    }
}

pub mod query {
    #[cfg(feature = "pg")]
    pub use kova_pg::query::{Delete, Insert, Select, Update};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::query::{Delete, Insert, Select, Update};
}

#[cfg(all(feature = "pg", feature = "sqlite"))]
compile_error!("Features 'pg' and 'sqlite' are mutually exclusive in Kova ORM.");
