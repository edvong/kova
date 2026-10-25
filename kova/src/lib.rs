mod macros;

pub use kova_core::{Error, Result};
pub use kova_derive::{Cte, FromRow, Model};

#[cfg(feature = "postgres")]
pub use kova_postgres::options::Options;

#[cfg(feature = "sqlite")]
pub use kova_sqlite::options::Options;

#[cfg(feature = "postgres")]
pub type Client<X = ()> = kova_core::database::Client<types::Engine, kova_core::types::Async, X>;

#[cfg(feature = "sqlite")]
pub type Client<X = ()> = kova_core::database::Client<types::Engine, kova_core::types::Sync, X>;

pub type Transaction<'b, 't, X = ()> = kova_core::database::Transaction<'b, 't, types::Engine, X>;

pub mod options {
    #[cfg(feature = "postgres")]
    pub use kova_postgres::options::RecyclingMethod;

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::options::InitConnection;
}

pub mod types {
    pub use kova_core::types::{
        ColumnMeta, Cte, Database, FromRow, IdentifierMeta, Model, Relation, Row,
    };

    #[cfg(feature = "postgres")]
    pub use kova_postgres::types::Postgres as Engine;

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::types::Sqlite as Engine;
}

pub mod codec {
    pub use crate::decode_column;
    pub use kova_core::dsl::{
        CanMath, CanString, Compatible, JsonType, JsonbType, Static,
    };
    pub use kova_core::types::DecodeColumn;

    #[cfg(feature = "postgres")]
    pub use kova_postgres::{
        dsl::{CanPgArray, CanPgJsonb},
        types::{Decode, Encode, Value},
    };

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::{
        dsl::CanSqliteJson,
        types::{Decode, Encode, Value},
    };
}

pub mod dsl {
    pub use kova_core::dsl::{Condition, Dynamic, Identifier, Table};

    #[cfg(feature = "postgres")]
    pub use kova_postgres::dsl::{Column, Constraint, LockMode, Node, WaitPolicy};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::dsl::{Column, Node};

    pub mod func {
        pub use kova_core::dsl::func::{FrameBound, FrameType, Wildcard, Window};

        #[cfg(feature = "postgres")]
        pub use kova_postgres::dsl::func::{
            // Standard
            Abs, Avg, Count, Length, Lower, Max, Min, Sum, Upper,

            // Extension
            All, Any, ArrayAgg, CumeDist, DenseRank, GenerateSeries, JsonbAgg, JsonbArrayElements,
            JsonbBuildObject, JsonbEach, JsonbEachText, JsonbObject, JsonbObjectSingle, Lag, Lead,
            Math, Ntile, PercentRank, Rank, RowNumber, RowToJson, Unnest,
        };

        #[cfg(feature = "sqlite")]
        pub use kova_sqlite::dsl::func::{
            // Standard
            Abs, Avg, Count, Length, Lower, Max, Min, Sum, Upper,

            // Extension
            Bm25, GroupConcat, Highlight, Json, JsonEach, JsonExtract, JsonGroupArray, JsonObject,
            JsonTree, JsonbExtract, JsonbGroupArray, JsonbObject, Math, Snippet,
        };
    }
}

pub mod query {
    #[cfg(feature = "postgres")]
    pub use kova_postgres::query::{Delete, Insert, Select, Statement, Update};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::query::{Delete, Insert, Select, Statement, Update};
}

#[cfg(all(feature = "postgres", feature = "sqlite"))]
compile_error!("Features 'postgres' and 'sqlite' are mutually exclusive.");

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Features 'rustls' and 'native-tls' are mutually exclusive.");

#[cfg(all(feature = "sqlite", any(feature = "rustls", feature = "native-tls")))]
compile_error!("SQLite does not support network TLS features.");