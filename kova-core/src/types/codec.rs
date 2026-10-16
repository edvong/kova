use crate::Result;
use crate::types::{Database, FromValue, IntoValue, Value};

pub trait Encode<'a, DB: Database, X> {
    fn encode(self) -> Result<DB::Value<'a>>;
}

pub trait Decode<'a, DB: Database, X>: Sized {
    fn decode(value: DB::Value<'a>) -> Result<Self>;
}

impl<'a, DB: Database, X, T> Encode<'a, DB, X> for Option<T>
    where
        T: Encode<'a, DB, X> {
    fn encode(self) -> Result<DB::Value<'a>> {
        match self {
            Some(val) => val.encode(),
            None => Ok(DB::Value::none()),
        }
    }
}

impl<'a, 'b, DB: Database, X, T> Encode<'a, DB, X> for &'b Option<T>
    where
        &'b T: Encode<'a, DB, X> {
    fn encode(self) -> Result<DB::Value<'a>> {
        match self {
            Some(val) => val.encode(),
            None => Ok(DB::Value::none()),
        }
    }
}

impl<'a, DB: Database, X, T> Decode<'a, DB, X> for Option<T>
    where
        T: Decode<'a, DB, X> {
    fn decode(value: DB::Value<'a>) -> Result<Self> {
        if value.is_none() {
            Ok(None)
        } else {
            T::decode(value).map(Some)
        }
    }
}

impl<'a, DB: Database, X, T> Decode<'a, DB, X> for Box<T>
    where
        T: Decode<'a, DB, X> {
    #[inline(always)]
    fn decode(value: DB::Value<'a>) -> Result<Self> {
        T::decode(value).map(Box::new)
    }
}

impl<'a, DB: Database, X, const N: usize> Decode<'a, DB, X> for [u8; N]
    where
        [u8; N]: FromValue<DB> {
    #[inline(always)]
    fn decode(value: DB::Value<'a>) -> Result<Self> {
        FromValue::from_value(value)
    }
}

impl<'a, 'b, DB: Database, X> Encode<'a, DB, X> for &'b str
    where
        &'b str: IntoValue<'a, DB> {
    fn encode(self) -> Result<DB::Value<'a>> {
        self.into_value()
    }
}

impl<'a, 'b, DB: Database, X> Encode<'a, DB, X> for &'b [u8]
    where
        &'b [u8]: IntoValue<'a, DB> {
    fn encode(self) -> Result<DB::Value<'a>> {
        self.into_value()
    }
}

impl<'a, DB: Database, X, const N: usize> Encode<'a, DB, X> for [u8; N]
    where
        [u8; N]: IntoValue<'a, DB>,
{
    fn encode(self) -> Result<DB::Value<'a>> {
        self.into_value()
    }
}

impl<'a, 'b, DB: Database, X, const N: usize> Encode<'a, DB, X> for &'b [u8; N]
    where
        &'b [u8; N]: IntoValue<'a, DB>,
{
    fn encode(self) -> Result<DB::Value<'a>> {
        self.into_value()
    }
}
