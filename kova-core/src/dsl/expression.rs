use crate::Result;
use crate::dsl::column::Column;
// use crate::types::json::Json;
use crate::types::{ColumnMeta, Database, ToValue};


#[derive(Debug, Clone)]
pub enum Expression<'a, DB: Database> {
    Column(ColumnMeta),
    Value(DB::Value<'a>),
    Error(String),
}

impl<'a, DB: Database> From<ColumnMeta> for Expression<'a, DB> {
    fn from(value: ColumnMeta) -> Self {
        Expression::Column(value)
    }
}

impl<'a, DB: Database> From<Result<DB::Value<'a>>> for Expression<'a, DB> {
    fn from(value: Result<DB::Value<'a>>) -> Self {
        match value {
            Ok(v) => Expression::Value(v),
            Err(e) => Expression::Error(e.to_string()),
        }
    }
}


pub trait IntoExpression<'a, T, DB: Database> {
    fn into_expression(self) -> Expression<'a, DB>;
}

impl<'a, T, DB: Database> IntoExpression<'a, T, DB> for Column<'a, T, DB> {
    fn into_expression(self) -> Expression<'a, DB> {
        Expression::Column(self.meta)
    }
}

impl<'a, T, DB> IntoExpression<'a, T, DB> for T where T: ToValue<'a, DB>, DB: Database {
    fn into_expression(self) -> Expression<'a, DB> {
        match self.to_value() {
            Ok(v) => Expression::Value(v),
            Err(e) => Expression::Error(e.to_string()),
        }
    }
}

impl<'a, T, DB> IntoExpression<'a, T, DB> for &'a T where T: ToValue<'a, DB>, DB: Database {
    fn into_expression(self) -> Expression<'a, DB> {
        Expression::from(self.to_value())
    }
}


impl<'a, DB> IntoExpression<'a, String, DB> for &'a str
where
    &'a str: ToValue<'a, DB>,
    DB: Database {
    fn into_expression(self) -> Expression<'a, DB> {
        Expression::from(self.to_value())
    }
}


impl<'a, DB> IntoExpression<'a, Vec<u8>, DB> for &'a [u8]
where
    &'a [u8]: ToValue<'a, DB>,
    DB: Database {
    fn into_expression(self) -> Expression<'a, DB> {
        Expression::from(self.to_value())
    }
}


// #[cfg(feature = "json")]
// impl<'a, T> IntoSqlExpr<Json<T>> for &'a T
// where
//     T: serde::Serialize,
// {
//     fn into_sql_expr(self) -> SqlExpr {
//         match serde_json::to_string(self) {
//             Ok(json_str) => SqlExpr::Value(Value::Text(json_str)),
//             Err(e) => SqlExpr::Error(e.to_string()),
//         }
//     }
// }
//
// /// Ergonomic bridge allowing JSON columns to directly accept owned Rust types (`T`)
// /// without requiring the user to manually wrap them in `Json(value)`.
// #[cfg(feature = "json")]
// impl<T> IntoSqlExpr<Json<T>> for T
// where
//     T: serde::Serialize,
// {
//     fn into_sql_expr(self) -> SqlExpr {
//         match serde_json::to_string(&self) {
//             Ok(json_str) => SqlExpr::Value(Value::Text(json_str)),
//             Err(e) => SqlExpr::Error(e.to_string()),
//         }
//     }
// }