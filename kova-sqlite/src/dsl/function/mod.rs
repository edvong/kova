mod group_concat;
mod json_extract;
mod json_group_array;
mod json_object;
mod jsonb_extract;
mod jsonb_group_array;
mod jsonb_object;
mod json;

use crate::types::Sqlite;
use kova_core::dsl::{Operand, Projection};

pub use group_concat::GroupConcat;
pub use json::Json;
pub use json_extract::JsonExtract;
pub use jsonb_extract::JsonbExtract;
pub use json_object::JsonObject;
pub use jsonb_object::JsonbObject;
pub use json_group_array::JsonGroupArray;
pub use jsonb_group_array::JsonbGroupArray;


pub enum ExtFunction<'a> {
    // Aggregated dialects
    GroupConcat(Box<Projection<'a, Sqlite, ()>>, Option<&'static str>), // (field, separator)

    JsonObject(Vec<Projection<'a, Sqlite, ()>>),
    JsonbObject(Vec<Projection<'a, Sqlite, ()>>),
    
    JsonGroupArray(Box<Projection<'a, Sqlite, ()>>),
    JsonbGroupArray(Box<Projection<'a, Sqlite, ()>>),

    // Scalar dialects
    JsonExtract(Box<Operand<'a, Sqlite>>, Box<Operand<'a, Sqlite>>),
    JsonbExtract(Box<Operand<'a, Sqlite>>, Box<Operand<'a, Sqlite>>),

    JsonValid(Box<Operand<'a, Sqlite>>),
    JsonType(Box<Operand<'a, Sqlite>>, Option<Box<Operand<'a, Sqlite>>>),

    // Convert the underlying SQLite JSONB Blob back to standard JSON Text
    Json(Box<Operand<'a, Sqlite>>),
}
