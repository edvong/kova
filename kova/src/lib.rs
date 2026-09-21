mod macros;

pub use kova_core::database::{Client, Transaction};
pub use kova_core::{Error, Result};
pub use kova_derive::{FromRow, Model};

#[cfg(feature = "pg")]
pub use kova_pg::options::Options;

#[cfg(feature = "sqlite")]
pub use kova_sqlite::options::Options;

pub mod options {
    #[cfg(feature = "pg")]
    pub use kova_pg::options::RecyclingMethod;
}

pub mod types {
    pub use kova_core::types::{
        ColumnMeta, Cte, Database, Decode, DecodeExt, Encode, FromColumn, FromRow, IdentifierMeta,
        Model, Relation, Row,
    };

    #[cfg(feature = "pg")]
    pub use kova_pg::types::{Postgres as Engine, Value};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::types::{Sqlite as Engine, Value};
}

pub mod dsl {
    pub use kova_core::dsl::{Column, Condition, Identifier};

    #[cfg(feature = "pg")]
    pub use kova_pg::dsl::{Constraint, LockMode, Subselect, WaitPolicy};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::dsl::Subselect;

    #[doc(hidden)]
    #[cfg(feature = "pg")]
    pub use kova_pg::dsl::ExtColumn;

    #[doc(hidden)]
    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::dsl::ExtColumn;

    pub mod func {
        pub use kova_core::dsl::func::{
            Abs, Avg, Count, CountAll, Length, Lower, Max, Min, Sum, Upper,
        };

        #[cfg(feature = "pg")]
        pub use kova_pg::dsl::func::{All, Any, ArrayAgg, JsonbAgg, JsonbObject};

        #[cfg(feature = "sqlite")]
        pub use kova_sqlite::dsl::func::{
            Bm25, GroupConcat, Highlight, Json, JsonExtract, JsonGroupArray, JsonObject,
            JsonbExtract, JsonbGroupArray, JsonbObject, Snippet,
        };
    }
}

pub mod query {
    #[cfg(feature = "pg")]
    pub use kova_pg::query::{Delete, Insert, Select, Update};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::query::{Delete, Insert, Select, Update};
}

#[cfg(all(feature = "pg", feature = "sqlite"))]
compile_error!("Features 'pg' and 'sqlite' are mutually exclusive.");

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Features 'rustls' and 'native-tls' are mutually exclusive.");

#[cfg(all(feature = "sqlite", any(feature = "rustls", feature = "native-tls")))]
compile_error!("SQLite does not support network TLS features.");
