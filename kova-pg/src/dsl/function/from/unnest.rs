use crate::dsl::function::from::{ExtTableFunction, IntoTableFunction};
use crate::dsl::private;
use crate::types::Postgres;
use kova_core::dsl::{Dynamic, IntoArg, SingleArgMarker, FromItem, VirtualColumn};
use std::ops::Deref;

pub struct Unnest<T = ()>(pub T);

pub struct UnnestSchema {
    pub unnest: VirtualColumn<Postgres, (), Dynamic>,
}

pub const UNNEST_DEFAULT: UnnestSchema = UnnestSchema {
    unnest: private::create_virtual_column("unnest", "unnest"),
};

#[allow(non_upper_case_globals)]
impl Unnest<()> {
    pub const unnest: VirtualColumn<Postgres, (), Dynamic> = UNNEST_DEFAULT.unnest;
}

impl<T> Deref for Unnest<T> {
    type Target = UnnestSchema;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &UNNEST_DEFAULT
    }
}

impl<T> SingleArgMarker for Unnest<T> {}

impl<'a, T, X> IntoArg<FromItem<'a, Postgres, X>> for Unnest<T>
    where
        T: IntoTableFunction<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, Postgres, X> {
        FromItem::from_function(ExtTableFunction::Unnest(self.0.into_table_function()), None)
    }
}
