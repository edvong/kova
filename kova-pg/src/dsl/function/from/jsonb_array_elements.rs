use crate::dsl::function::from::{ExtTableFunction, IntoTableFunction};
use crate::types::Postgres;
use kova_core::dsl::{IntoArg, SingleArgMarker, Table, VirtualColumn};

pub struct JsonbArrayElements<T = ()>(pub T);

#[allow(non_upper_case_globals)]
impl JsonbArrayElements<()> {
    pub const value: VirtualColumn = VirtualColumn::new("jsonb_array_elements", "value");
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
