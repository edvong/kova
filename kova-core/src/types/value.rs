use crate::types::Database;
use crate::Result;

pub trait ToValue<'a, DB: Database> {

    fn to_value(&self) -> Result<DB::Value<'a>>;
}

pub trait FromValue<DB: Database>: Sized {

    fn from_value(value: DB::Value<'_>) -> Result<Self>;
}