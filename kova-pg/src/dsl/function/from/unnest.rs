use crate::dsl::function::from::{ExtTableFunction, IntoTableFunction};
use crate::types::Postgres;
use kova_core::dsl::{IntoArg, SingleArgMarker, Table, VirtualColumn};

pub struct Unnest<T = ()>(pub T);

#[allow(non_upper_case_globals)]
impl Unnest<()> {
    pub const unnest: VirtualColumn = VirtualColumn::new("unnest", "unnest");
}

impl<T> SingleArgMarker for Unnest<T> {}

impl<'a, T, X> IntoArg<Table<'a, Postgres, X>> for Unnest<T>
    where
        T: IntoTableFunction<'a, X> {
    #[inline(always)]
    fn into_arg(self) -> Table<'a, Postgres, X> {
        Table::from_function(ExtTableFunction::Unnest(self.0.into_table_function()), None)
    }
}
