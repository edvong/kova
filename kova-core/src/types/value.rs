use crate::Result;
use crate::types::Database;

pub trait IntoValue<'a, DB: Database> {
    fn into_value(
        self) -> Result<DB::Value<'a>>;
}

pub trait FromValue<DB: Database>: Sized {
    fn from_value(value: DB::Value<'_>) -> Result<Self>;
}

pub trait Value {
    fn none() -> Self;
    fn is_none(&self) -> bool;
}

impl<'a, DB: Database, T: IntoValue<'a, DB>> IntoValue<'a, DB> for Option<T> {
    fn into_value(self) -> Result<DB::Value<'a>> {
        match self {
            Some(val) => val.into_value(),
            None => Ok(DB::Value::none()),
        }
    }
}

impl<DB: Database, T: FromValue<DB>> FromValue<DB> for Option<T> {
    fn from_value(value: DB::Value<'_>) -> Result<Self> {
        match value.is_none() {
            true => Ok(None),
            false => T::from_value(value).map(Some),
        }
    }
}

impl<'a, DB: Database, T: IntoValue<'a, DB>> IntoValue<'a, DB> for Box<T> {
    fn into_value(self) -> Result<DB::Value<'a>> {
        (*self).into_value()
    }
}

impl<DB: Database, T: FromValue<DB>> FromValue<DB> for Box<T> {
    fn from_value(value: DB::Value<'_>) -> Result<Self> {
        T::from_value(value).map(Box::new)
    }
}

// For RC/ARC, accept Only Ref Type.
