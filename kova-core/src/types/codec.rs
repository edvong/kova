use crate::Result;
use crate::types::Database;

pub trait Encode<'a, T, X>: Database {
    fn encode(val: T) -> Result<Self::Value<'a>>;
}

pub trait Decode<T, X>: Database {
    fn decode(val: Self::Value<'_>) -> Result<T>;
}

#[macro_export]
macro_rules! impl_platform_prim_codec {
    ($db:ty, $($prim:ty),+ $(,)?) => {
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

            impl<X> Decode<X> for $prim {
                #[inline(always)]
                fn decode(value: <$db as $crate::types::Database>::Value<'_>) -> $crate::Result<Self> {
                    use $crate::types::Value as _;
                    if value.is_none() {
                        return Err($crate::Error::UnexpectedNull(
                            concat!("Unexpected NULL for ", stringify!($prim)).into(),
                        ));
                    }
                    $crate::types::FromValue::from_value(value)
                }
            }

            impl<X> $crate::types::DecodeColumn<$db, X> for $prim where $prim: Decode<X> {
                #[inline(always)]
                fn extract_next(
                    row: &<$db as $crate::types::Database>::Row<'_>, idx: &mut usize,
                ) -> $crate::Result<Self> {
                    <Self as $crate::types::DecodeColumn<$db, X>>::extract_value(row, idx)
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_platform_encode_only {
    ($db:ty, $($prim:ty),+ $(,)?) => {
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
        )+
    };
}

#[macro_export]
macro_rules! impl_platform_decode_only {
    ($db:ty, $($prim:ty),+ $(,)?) => {
        $(
            impl<X> Decode<X> for $prim {
                #[inline(always)]
                fn decode(value: <$db as $crate::types::Database>::Value<'_>) -> $crate::Result<Self> {
                    use $crate::types::Value as _;
                    if value.is_none() {
                        return Err($crate::Error::UnexpectedNull(
                            concat!("Unexpected NULL for ", stringify!($prim)).into(),
                        ));
                    }
                    $crate::types::FromValue::from_value(value)
                }
            }

            impl<X> $crate::types::DecodeColumn<$db, X> for $prim where $prim: Decode<X> {
                #[inline(always)]
                fn extract_next(row: &<$db as $crate::types::Database>::Row<'_>, idx: &mut usize)
                    -> $crate::Result<Self> {
                    <Self as $crate::types::DecodeColumn<$db, X>>::extract_value(row, idx)
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_platform_codec {
    ($db:ty, $($prim:ty),+ $(,)?) => {
        impl<'a, T, X> $crate::types::Encode<'a, T, X> for $db where T: Encode<'a, X> {
            #[inline(always)]
            fn encode(val: T) -> $crate::Result<Self::Value<'a>> {
                <T as Encode<'a, X>>::encode(val)
            }
        }

        impl<T, X> $crate::types::Decode<T, X> for $db where T: Decode<X> {
            #[inline(always)]
            fn decode(val: Self::Value<'_>) -> $crate::Result<T> {
                use $crate::types::Value as _;
                let is_null = val.is_none();
                <T as Decode<X>>::decode(val).map_err(|e| match is_null {
                    true if !matches!(e, $crate::Error::UnexpectedNull(_)) => {
                        $crate::Error::UnexpectedNull(std::any::type_name::<T>().into())
                    }
                    _ => e,
                })
            }
        }

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

        impl<X, T> Decode<X> for Option<T> where T: Decode<X> {
            #[inline(always)]
            fn decode(value: <$db as $crate::types::Database>::Value<'_>) -> $crate::Result<Self> {
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

        impl<X, T> Decode<X> for Box<T> where T: Decode<X> {
            #[inline(always)]
            fn decode(value: <$db as $crate::types::Database>::Value<'_>) -> $crate::Result<Self> {
                T::decode(value).map(Box::new)
            }
        }

        impl<'a, 'b, X> Encode<'a, X> for &'b str where &'b str: $crate::types::IntoValue<'a, $db> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                $crate::types::IntoValue::into_value(self)
            }
        }

        impl<'a, 'b, 'c, X> Encode<'a, X> for &'c &'b str where &'b str: $crate::types::IntoValue<'a, $db> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                $crate::types::IntoValue::into_value(*self)
            }
        }

        impl<'a, 'b, X> Encode<'a, X> for &'b [u8] where &'b [u8]: $crate::types::IntoValue<'a, $db> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                $crate::types::IntoValue::into_value(self)
            }
        }

        impl<'a, 'b, 'c, X> Encode<'a, X> for &'c &'b [u8] where &'b [u8]: $crate::types::IntoValue<'a, $db> {
            #[inline(always)]
            fn encode(self) -> $crate::Result<<$db as $crate::types::Database>::Value<'a>> {
                $crate::types::IntoValue::into_value(*self)
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

        impl<X, const N: usize> Decode<X> for [u8; N]
            where
                [u8; N]: $crate::types::FromValue<$db> {
            #[inline(always)]
            fn decode(value: <$db as $crate::types::Database>::Value<'_>) -> $crate::Result<Self> {
                use $crate::types::Value as _;
                if value.is_none() {
                    return Err($crate::Error::UnexpectedNull("Unexpected NULL for [u8; N]".into()));
                }
                $crate::types::FromValue::from_value(value)
            }
        }

        $crate::impl_platform_prim_codec!($db, $($prim),+);
    };
}
