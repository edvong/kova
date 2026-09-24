mod fts5;
mod group_concat;
mod json;
mod json_extract;
mod json_group_array;
mod json_object;
mod jsonb_extract;
mod jsonb_group_array;
mod jsonb_object;
mod from;

use crate::types::Sqlite;
use kova_core::dsl::{Operand, Projection};

pub use group_concat::GroupConcat;
pub use json::Json;
pub use json_extract::JsonExtract;
pub use json_group_array::JsonGroupArray;
pub use json_object::JsonObject;
pub use jsonb_extract::JsonbExtract;
pub use jsonb_group_array::JsonbGroupArray;
pub use jsonb_object::JsonbObject;

pub use from::ExtTableFunction;
pub use from::{JsonEach, JsonEachSchema, JSON_EACH_DEFAULT};
pub use from::{JsonTree, JsonTreeSchema, JSON_TREE_DEFAULT};


pub use fts5::bm25::Bm25;
pub use fts5::highlight::Highlight;
pub use fts5::snippet::Snippet;

pub enum ExtFunction<'a> {
    // Aggregated dialects
    GroupConcat(Box<Projection<'a, Sqlite, ()>>, Option<&'static str>), // (field, separator)

    JsonObject(Vec<Projection<'a, Sqlite, ()>>),
    JsonbObject(Vec<Projection<'a, Sqlite, ()>>),

    JsonGroupArray(Box<Projection<'a, Sqlite, ()>>),
    JsonbGroupArray(Box<Projection<'a, Sqlite, ()>>),

    // Scalar dialects
    JsonExtract(Opnd<'a>, Opnd<'a>),
    JsonbExtract(Opnd<'a>, Opnd<'a>),

    JsonValid(Opnd<'a>),
    JsonType(Opnd<'a>, Opnd<'a>),

    // Convert the underlying SQLite JSONB Blob back to standard JSON Text
    Json(Opnd<'a>),

    // table name
    Bm25(&'static str),
    // table name, column index, left label, right label
    Highlight(&'static str, i32, Opnd<'a>, Opnd<'a>),
    // table name, column index, left label, right label, ellipsis, max tokens
    Snippet(&'static str, i32, Opnd<'a>, Opnd<'a>, Opnd<'a>, i32),
}

type Opnd<'a> = Box<Operand<'a, Sqlite>>;
