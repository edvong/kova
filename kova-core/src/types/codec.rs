use crate::Result;
use crate::types::Database;

pub trait Encode<'a, T, X>: Database {
    fn encode(val: T) -> Result<Self::Value<'a>>;
}

pub trait Decode<'a, T, X>: Database {
    fn decode(val: Self::Value<'a>) -> Result<T>;
}

#[macro_export]
macro_rules! impl_platform_codec {
    ($db:ty, $($prim:ty),+ $(,)?) => {
        // Bridge Implementation: Unify platform-side Encode / Decode into kova-core's Database Protocol
        impl<'a, T, X> $crate::types::Encode<'a, T, X> for $db where T: Encode<'a, X> {
            #[inline(always)]
            fn encode(val: T) -> $crate::Result<Self::Value<'a>> {
                <T as Encode<'a, X>>::encode(val)
            }
        }

        impl<'a, T, X> $crate::types::Decode<'a, T, X> for $db where T: Decode<'a, X> {
            #[inline(always)]
            fn decode(val: Self::Value<'a>) -> $crate::Result<T> {
                <T as Decode<'a, X>>::decode(val)
            }
        }

        // General Container and Slice Implementation (Option, Box, &str, &[u8], [u8; N])
        impl<'a, X, T> Encode<'a, X> for Option<T> where T: Encode<'a, X> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                use $crate::types::Value as _;
                match self {
                    Some(val) => val.encode(),
                    None => Ok(<$db as $crate::types::Database>::Value::none()),
                }
            }
        }

        impl<'a, 'b, X, T> Encode<'a, X> for &'b Option<T> where &'b T: Encode<'a, X> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                use $crate::types::Value as _;
                match self {
                    Some(val) => val.encode(),
                    None => Ok(<$db as $crate::types::Database>::Value::none()),
                }
            }
        }

        impl<'a, X, T> Decode<'a, X> for Option<T> where T: Decode<'a, X> {
            #[inline(always)]
            fn decode(value: <$db as $crate::types::Database>::Value<'a>) -> $crate::Result<Self> {
                use $crate::types::Value as _;
                if value.is_none() {
                    Ok(None)
                } else {
                    T::decode(value).map(Some)
                }
            }
        }

        impl<'a, X, T> Encode<'a, X> for Box<T> where T: Encode<'a, X> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                (*self).encode()
            }
        }

        impl<'a, 'b, X, T> Encode<'a, X> for &'b Box<T> where &'b T: Encode<'a, X> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                (&**self).encode()
            }
        }

        impl<'a, X, T> Decode<'a, X> for Box<T> where T: Decode<'a, X> {
            #[inline(always)]
            fn decode(value: <$db as $crate::types::Database>::Value<'a>) -> $crate::Result<Self> {
                T::decode(value).map(Box::new)
            }
        }

        impl<'a, 'b, X> Encode<'a, X> for &'b str where &'b str: $crate::types::IntoValue<'a, $db> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                $crate::types::IntoValue::into_value(self)
            }
        }

        impl<'a, 'b, X> Encode<'a, X> for &'b [u8] where &'b [u8]: $crate::types::IntoValue<'a, $db> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                $crate::types::IntoValue::into_value(self)
            }
        }

        impl<'a, X, const N: usize> Encode<'a, X> for [u8; N]
            where
                [u8; N]: $crate::types::IntoValue<'a, $db> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                $crate::types::IntoValue::into_value(self)
            }
        }

        impl<'a, 'b, X, const N: usize> Encode<'a, X> for &'b [u8; N]
            where
                &'b [u8; N]: $crate::types::IntoValue<'a, $db> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                $crate::types::IntoValue::into_value(self)
            }
        }

        impl<'a, X, const N: usize> Decode<'a, X> for [u8; N]
            where
                [u8; N]: $crate::types::FromValue<$db> {
            #[inline(always)]
            fn decode(value: <$db as $crate::types::Database>::Value<'a>) -> $crate::Result<Self> {
                $crate::types::FromValue::from_value(value)
            }
        }

        // Native basic type Encode / Decode / FromColumn implementations
        $(
            impl<'a, X> Encode<'a, X> for $prim {
                #[inline(always)]
                fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                    $crate::types::IntoValue::into_value(self)
                }
            }

            impl<'a, 'b: 'a, X> Encode<'a, X> for &'b $prim {
                #[inline(always)]
                fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                    $crate::types::IntoValue::into_value(self)
                }
            }

            impl<'a, X> Decode<'a, X> for $prim {
                #[inline(always)]
                fn decode(value: <$db as $crate::types::Database>::Value<'a>) -> $crate::Result<Self> {
                    $crate::types::FromValue::from_value(value)
                }
            }

            impl<'a, X> $crate::types::FromColumn<'a, $db, X> for $prim where $prim: Decode<'a, X> {
                #[inline(always)]
                fn extract_next(
                    row: &<$db as $crate::types::Database>::Row<'a>,
                    idx: &mut usize,
                ) -> $crate::Result<Self> {
                    Self::extract_value(row, idx)
                }
            }
        )+
    };
}

