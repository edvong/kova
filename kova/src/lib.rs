mod macros;

pub use kova_core::database::{Client, Transaction};
pub use kova_core::{Error, Result};
pub use kova_derive::{Cte, FromRow, Model};

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
        ColumnMeta, Cte, Database, FromRow, IdentifierMeta, Model, Relation, Row,
    };

    #[cfg(feature = "pg")]
    pub use kova_pg::types::Postgres as Engine;

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::types::Sqlite as Engine;
}

pub mod codec {
    pub use crate::decode_column;
    pub use kova_core::types::DecodeColumn;
    pub use kova_core::dsl::{
        CanMath, CanString, Compatible, JsonType, JsonbType, Static,
    };

    #[cfg(feature = "pg")]
    pub use kova_pg::{
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

    #[cfg(feature = "pg")]
    pub use kova_pg::dsl::{Column, Constraint, LockMode, Node, WaitPolicy};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::dsl::{Column, Node};

    pub mod func {
        pub use kova_core::dsl::func::{FrameBound, FrameType, Wildcard, Window};

        #[cfg(feature = "pg")]
        pub use kova_pg::dsl::func::{
            // Standard
            Abs, Avg, Count, Length, Lower, Max, Min, Sum, Upper,

            // Extension
            All, Any, ArrayAgg, CumeDist, DenseRank, GenerateSeries, JsonbAgg, JsonbArrayElements,
            JsonbBuildObject, JsonbEach, JsonbEachText, JsonbObject, JsonbObjectSingle, Lag, Lead,
            Ntile, PercentRank, Rank, RowNumber, RowToJson, Unnest,
        };

        #[cfg(feature = "sqlite")]
        pub use kova_sqlite::dsl::func::{
            // Standard
            Abs, Avg, Count, Length, Lower, Max, Min, Sum, Upper,

            // Extension
            Bm25, GroupConcat, Highlight, Json, JsonEach, JsonExtract, JsonGroupArray, JsonObject,
            JsonTree, JsonbExtract, JsonbGroupArray, JsonbObject, Snippet,
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
