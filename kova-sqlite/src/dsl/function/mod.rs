mod aggregate;
mod can;
// mod from;
mod fts5;
mod json;
mod standard;
mod srf;

use crate::types::Sqlite;
use kova_core::dsl::{Operand, Projection};

pub use self::aggregate::{GroupConcat, JsonGroupArray, JsonbGroupArray};
pub use self::srf::{ExtSrfFunction, IntoSrfFunction, JsonEach, JsonSrfArgs, JsonTree};
pub use self::fts5::{Bm25, Highlight, InnerBm25, InnerHighlight, InnerSnippet, Snippet};
pub use self::json::{
    InnerJsonExtract, Json, JsonExtract, JsonExtractMarker, JsonObject, JsonbExtract,
    JsonbExtractMarker, JsonbObject,
};
pub use self::standard::*;

pub enum ExtFunction<'a> {
    // Aggregated dialects
    GroupConcat(Opnd<'a>, Option<&'static str>), // (field, separator)
    JsonGroupArray(Opnd<'a>),
    JsonbGroupArray(Opnd<'a>),

    JsonObject(Vec<Projection<'a, Sqlite, ()>>),
    JsonbObject(Vec<Projection<'a, Sqlite, ()>>),

    // Scalar dialects
    JsonExtract(Opnd<'a>, Opnd<'a>),
    JsonbExtract(Opnd<'a>, Opnd<'a>),

    JsonValid(Opnd<'a>),
    JsonType(Opnd<'a>, Opnd<'a>),

    // Convert the underlying SQLite JSONB Blob back to standard JSON Text
    Json(Opnd<'a>),

    // table name
    Bm25(&'static str, Vec<f64>),
    // table name, column index, left label, right label
    Highlight(&'static str, i32, Opnd<'a>, Opnd<'a>),
    // table name, column index, left label, right label, ellipsis, max tokens
    Snippet(&'static str, i32, Opnd<'a>, Opnd<'a>, Opnd<'a>, i32),
}

type Opnd<'a> = Box<Operand<'a, Sqlite>>;
