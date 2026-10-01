use crate::dsl::function::from::{ExtTableFunction, IntoTableFunction};
use crate::dsl::private::create_virtual_column;
use crate::types::Postgres;
use kova_core::dsl::{Dynamic, IntoArg, SingleArgMarker, Table, VirtualColumn};
use std::ops::Deref;

pub struct JsonbArrayElements<T = ()>(pub T);

pub struct JsonbArrayElementsSchema {
    pub value: VirtualColumn<Postgres, (), Dynamic>,
}

#[allow(non_upper_case_globals)]
impl JsonbArrayElements<()> {
    pub const value: VirtualColumn<Postgres, (), Dynamic> = JSONB_ARRAY_ELEMENTS_DEFAULT.value;
}

pub const JSONB_ARRAY_ELEMENTS_DEFAULT: JsonbArrayElementsSchema = JsonbArrayElementsSchema {
    value: create_virtual_column("jsonb_array_elements", "value"),
};

impl<T> Deref for JsonbArrayElements<T> {
    type Target = JsonbArrayElementsSchema;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &JSONB_ARRAY_ELEMENTS_DEFAULT
    }
}

impl<T> SingleArgMarker for JsonbArrayElements<T> {}

impl<'a, T, X> IntoArg<Table<'a, Postgres, X>> for JsonbArrayElements<T>
    where
        T: IntoTableFunction<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> Table<'a, Postgres, X> {
        Table::from_function(
            ExtTableFunction::JsonbArrayElements(self.0.into_table_function()),
            None,
        )
    }
}
