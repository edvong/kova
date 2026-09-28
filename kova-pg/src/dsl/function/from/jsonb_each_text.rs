use crate::dsl::function::from::{ExtTableFunction, IntoTableFunction};
use crate::types::Postgres;
use kova_core::dsl::{IntoArg, SingleArgMarker, Table, VirtualColumn};

pub struct JsonbEachText<T = ()>(pub T);

#[allow(non_upper_case_globals)]
impl JsonbEachText<()> {
    pub const key: VirtualColumn = VirtualColumn::new("jsonb_each_text", "key");
    pub const value: VirtualColumn = VirtualColumn::new("jsonb_each_text", "value");
}

impl<T> SingleArgMarker for JsonbEachText<T> {}

impl<'a, T, X> IntoArg<Table<'a, Postgres, X>> for JsonbEachText<T>
    where
        T: IntoTableFunction<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> Table<'a, Postgres, X> {
        Table::from_function(ExtTableFunction::JsonbEachText(self.0.into_table_function()), None)
    }
}
