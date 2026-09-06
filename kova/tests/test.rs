use std::collections::HashMap;
use kova_core::dsl::{IntoOperand, Operand};
use kova::types::Engine;
use kova_core::types::FromColumn;

mod pg;
mod sqlite;


pub struct MyCodec;

// impl<'a> IntoOperand<'a, HashMap<String, String>, Engine, MyCodec> for HashMap<String, String> {
//     fn into_operand(self) -> Operand<'a, Engine> {
//         todo!()
//         // let json_str = serde_json::to_string(&self).unwrap();
//         // Operand::Value(kova::types::Value::Text(std::borrow::Cow::Owned(json_str)))
//     }
// }
//
// impl<'a> FromColumn<'a, Engine, MyCodec> for HashMap<String, String> {
//     fn extract(row: &<Engine as kova_core::types::Database>::Row<'a>) -> kova_core::Result<Self> {
//
//
//         todo!()
//     }
// }