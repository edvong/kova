
pub use kova_derive::{FromRow, Model};
pub use kova_core::{Error, Result};
// Client, Transaction

pub mod types {
    pub use kova_core::types::{Database, FromRow, Model, FromValue, IntoValue, ColumnMeta};

    #[cfg(feature = "pg")]
    pub use kova_pg::types::{Postgres as Engine};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::types::{Sqlite as Engine};

    #[cfg(feature = "json")]
    pub use kova_core::Json;
}

pub mod dsl {

    pub use kova_core::dsl::{Column, Condition};

    #[cfg(feature = "pg")]
    pub use kova_pg::dsl::{
        LockMode, WaitPolicy,
    };

    #[doc(hidden)]
    #[cfg(feature = "pg")]
    pub use kova_pg::dsl::Column as ExtColumn;



    #[doc(hidden)]
    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::dsl::Column as ExtColumn;


    pub mod func {
        pub use kova_core::dsl::func::{
            // Function, Quantifier,
            CountAll, Count, Abs, Avg, Length, Lower, Max, Min, Sum, Upper,
        };

        #[cfg(feature = "pg")]
        pub use kova_pg::dsl::func::{ArrayAgg};
    }
}

pub mod query {
    #[cfg(feature = "pg")]
    pub use kova_pg::query::{Select};

    #[cfg(feature = "sqlite")]
    pub use kova_sqlite::query::{Select};
}


#[cfg(all(feature = "pg", feature = "sqlite"))]
compile_error!("Features 'pg' and 'sqlite' are mutually exclusive in Kova ORM.");
