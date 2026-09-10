use crate::Result;
use crate::types::{Database, IntoValue, Value};

// Target: The target type, by default, is equal to Self.
// this is the heart of securing strong type constraints for Column<T>
pub trait Encode<'a, DB: Database, X, Target = Self> {
    fn encode(self) -> Result<DB::Value<'a>>;
}

pub trait Decode<'a, DB: Database, X>: Sized {
    fn decode(value: DB::Value<'a>) -> Result<Self>;
}

// // Native types automatically implement Encode
// impl<'a, T, DB, X> Encode<'a, DB, X, T> for T
// where
//     DB: Database,
//     T: IntoValue<'a, DB>,
// {
//     fn encode(self) -> Result<DB::Value<'a>> {
//         self.into_value()
//     }
// }
//
// // Native types automatically implement Decode
// impl<'a, T, DB, X> Decode<'a, DB, X> for T
// where
//     DB: Database,
//     T: FromValue<DB>,
// {
//     fn decode(value: DB::Value<'a>) -> Result<Self> {
//         T::from_value(value)
//     }
// }

impl<'a, DB: Database, X, T> Encode<'a, DB, X, Option<T>> for Option<T>
where
    T: Encode<'a, DB, X, T>,
{
    fn encode(self) -> Result<DB::Value<'a>> {
        match self {
            Some(val) => val.encode(),
            None => Ok(DB::Value::none()),
        }
    }
}

impl<'a, 'b, DB: Database, X, T> Encode<'a, DB, X, Option<T>> for &'b Option<T>
where
    &'b T: Encode<'a, DB, X, T>,
{
    fn encode(self) -> Result<DB::Value<'a>> {
        match self {
            Some(val) => val.encode(),
            None => Ok(DB::Value::none()),
        }
    }
}

impl<'a, DB: Database, X, T> Decode<'a, DB, X> for Option<T>
where
    T: Decode<'a, DB, X>,
{
    fn decode(value: DB::Value<'a>) -> Result<Self> {
        if value.is_none() {
            Ok(None)
        } else {
            T::decode(value).map(Some)
        }
    }
}

impl<'a, 'b, DB: Database, X> Encode<'a, DB, X, String> for &'b str
where
    &'b str: IntoValue<'a, DB>,
{
    fn encode(self) -> Result<DB::Value<'a>> {
        self.into_value()
    }
}

impl<'a, 'b, DB: Database, X> Encode<'a, DB, X, Vec<u8>> for &'b [u8]
where
    &'b [u8]: IntoValue<'a, DB>,
{
    fn encode(self) -> Result<DB::Value<'a>> {
        self.into_value()
    }
}

impl<'a, DB: Database, X, const N: usize> Encode<'a, DB, X, [u8; N]> for [u8; N]
where
    [u8; N]: IntoValue<'a, DB>,
{
    fn encode(self) -> Result<DB::Value<'a>> {
        self.into_value()
    }
}

impl<'a, 'b, DB: Database, X, const N: usize> Encode<'a, DB, X, [u8; N]> for &'b [u8; N]
where
    &'b [u8; N]: IntoValue<'a, DB>,
{
    fn encode(self) -> Result<DB::Value<'a>> {
        self.into_value()
    }
}

#[doc(hidden)]
pub trait DecodeExt<'a, DB: Database, X>: Decode<'a, DB, X> {
    #[inline(always)]
    fn decode_opt(value: DB::Value<'a>) -> Result<Option<Self>> {
        match Value::is_none(&value) {
            true => Ok(None),
            false => Self::decode(value).map(Some),
        }
    }

    #[inline(always)]
    fn decode_box(value: DB::Value<'a>) -> Result<Box<Self>> {
        Self::decode(value).map(Box::new)
    }
}

// Any type that implements Decode automatically gains the DecodeExt capability
impl<'a, DB, X, T> DecodeExt<'a, DB, X> for T
where
    DB: Database,
    T: Decode<'a, DB, X>,
{
}
