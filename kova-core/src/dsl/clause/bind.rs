use crate::Result;
use crate::dsl::{IntoArg, SingleArgMarker};
use crate::types::{Database, IntoValue};

pub struct Bind<'a, DB: Database>(Result<DB::Value<'a>>);

pub struct BindClause<'a, DB: Database> {
    pub(crate) params: Vec<Bind<'a, DB>>,
}

impl<'a, DB: Database> BindClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { params: Vec::new() }
    }

    pub fn set(&mut self, args: Vec<Bind<'a, DB>>) {
        self.params = args;
    }

    pub fn to_values(self) -> Result<Vec<DB::Value<'a>>> {
        let mut values = Vec::with_capacity(self.params.len());
        for p in self.params {
            values.push(p.0?);
        }
        Ok(values)
    }
}

impl<'a, T: ?Sized> SingleArgMarker for &'a T where T: SingleArgMarker {}

impl<'a, 'b, DB: Database, T: ?Sized> IntoArg<Bind<'a, DB>> for &'b T
    where
        &'b T: IntoValue<'a, DB>,
        T: SingleArgMarker {
    fn into_arg(self) -> Bind<'a, DB> {
        Bind(self.into_value())
    }
}

impl SingleArgMarker for [u8] {}
// (&[u8] is automatically supported because of the &'a T penetration above)

impl<const N: usize> SingleArgMarker for [u8; N] {}

impl<'a, const N: usize, DB: Database> IntoArg<Bind<'a, DB>> for [u8; N]
    where
        [u8; N]: IntoValue<'a, DB>,
{
    fn into_arg(self) -> Bind<'a, DB> {
        Bind(self.into_value())
    }
}

impl<T> SingleArgMarker for Option<T> where T: SingleArgMarker {}

impl<'a, DB: Database, T> IntoArg<Bind<'a, DB>> for Option<T>
    where
        Option<T>: IntoValue<'a, DB>,
        T: SingleArgMarker {
    fn into_arg(self) -> Bind<'a, DB> {
        Bind(self.into_value())
    }
}

impl<T> SingleArgMarker for Box<T> where T: SingleArgMarker {}

impl<'a, DB: Database, T> IntoArg<Bind<'a, DB>> for Box<T>
    where
        Box<T>: IntoValue<'a, DB>,
        T: SingleArgMarker {
    fn into_arg(self) -> Bind<'a, DB> {
        Bind(self.into_value())
    }
}

macro_rules! impl_native_bind {
    ($($T:ty),+) => {
        $(
            impl $crate::dsl::SingleArgMarker for $T {}

            impl<'a, DB: $crate::types::Database> $crate::dsl::IntoArg<Bind<'a, DB>> for $T
                where
                    $T: $crate::types::IntoValue<'a, DB> {
                fn into_arg(self) -> Bind<'a, DB> { Bind(self.into_value()) }
            }
        )+
    };
}

impl_native_bind!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, bool, String);
