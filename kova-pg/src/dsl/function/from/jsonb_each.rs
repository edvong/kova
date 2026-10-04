use crate::dsl::function::from::{ExtTableFunction, IntoTableFunction};
use crate::dsl::private::create_virtual_column;
use crate::types::Postgres;
use kova_core::dsl::{Dynamic, IntoArg, SingleArgMarker, FromItem, VirtualColumn};
use std::ops::Deref;

// Accept Column or Operand
pub struct JsonbEach<T>(pub T);

#[allow(non_upper_case_globals)]
impl JsonbEach<()> {
    pub const key: VirtualColumn<Postgres, String> = JSONB_EACH_DEFAULT.key;
    pub const value: VirtualColumn<Postgres, (), Dynamic> = JSONB_EACH_DEFAULT.value;
}

pub struct JsonbEachSchema {
    pub key: VirtualColumn<Postgres, String>,
    pub value: VirtualColumn<Postgres, (), Dynamic>,
}

pub const JSONB_EACH_DEFAULT: JsonbEachSchema = JsonbEachSchema {
    key: create_virtual_column("jsonb_each", "key"),
    value: create_virtual_column("jsonb_each", "value"),
};

impl<T> Deref for JsonbEach<T> {
    type Target = JsonbEachSchema;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &JSONB_EACH_DEFAULT
    }
}

impl<T> SingleArgMarker for JsonbEach<T> {}

impl<'a, T, X> IntoArg<FromItem<'a, Postgres, X>> for JsonbEach<T>
    where
        T: IntoTableFunction<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, Postgres, X> {
        FromItem::from_function(ExtTableFunction::JsonbEach(self.0.into_table_function()), None)
    }
}
