use crate::dsl::function::from::{ExtTableFunction, IntoTableFunction};
use crate::dsl::private::create_virtual_column;
use crate::types::Postgres;
use kova_core::dsl::{Dynamic, IntoArg, SingleArgMarker, FromItem, VirtualColumn};
use std::ops::Deref;

pub struct JsonbEachText<T = ()>(pub T);

#[allow(non_upper_case_globals)]
impl JsonbEachText<()> {
    pub const key: VirtualColumn<Postgres, String> = JSONB_EACH_TEXT_DEFAULT.key;
    pub const value: VirtualColumn<Postgres, (), Dynamic> = JSONB_EACH_TEXT_DEFAULT.value;
}

pub struct JsonbEachTextSchema {
    pub key: VirtualColumn<Postgres, String>,
    pub value: VirtualColumn<Postgres, (), Dynamic>,
}

pub const JSONB_EACH_TEXT_DEFAULT: JsonbEachTextSchema = JsonbEachTextSchema {
    key: create_virtual_column("jsonb_each_text", "key"),
    value: create_virtual_column("jsonb_each_text", "value"),
};

impl<T> Deref for JsonbEachText<T> {
    type Target = JsonbEachTextSchema;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &JSONB_EACH_TEXT_DEFAULT
    }
}

impl<T> SingleArgMarker for JsonbEachText<T> {}

impl<'a, T, X> IntoArg<FromItem<'a, Postgres, X>> for JsonbEachText<T>
    where
        T: IntoTableFunction<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> FromItem<'a, Postgres, X> {
        FromItem::from_function(ExtTableFunction::JsonbEachText(self.0.into_table_function()), None)
    }
}
