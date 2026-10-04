use crate::dsl::function::ExtTableFunction;
use crate::dsl::function::from::IntoTableFuncArgs;
use crate::dsl::private::create_virtual_column;
use crate::types::Sqlite;
use kova_core::dsl::{Dynamic, IntoArg, SingleArgMarker, FromItem, VirtualColumn};
use std::ops::Deref;

pub struct JsonEach<T = ()>(pub T);

#[allow(non_upper_case_globals)]
impl JsonEach<()> {
    pub const key: VirtualColumn<Sqlite, String> = JSON_EACH_DEFAULT.key;
    pub const value: VirtualColumn<Sqlite, (), Dynamic> = JSON_EACH_DEFAULT.value;
    pub const type_: VirtualColumn<Sqlite, String> = JSON_EACH_DEFAULT.type_;
    pub const atom: VirtualColumn<Sqlite, (), Dynamic> = JSON_EACH_DEFAULT.atom;
    pub const id: VirtualColumn<Sqlite, i64> = JSON_EACH_DEFAULT.id;
    pub const parent: VirtualColumn<Sqlite, i64> = JSON_EACH_DEFAULT.parent;
    pub const fullkey: VirtualColumn<Sqlite, String> = JSON_EACH_DEFAULT.fullkey;
    pub const path: VirtualColumn<Sqlite, String> = JSON_EACH_DEFAULT.path;
}

pub struct JsonEachSchema {
    pub key: VirtualColumn<Sqlite, String>,
    pub value: VirtualColumn<Sqlite, (), Dynamic>,
    pub type_: VirtualColumn<Sqlite, String>,
    pub atom: VirtualColumn<Sqlite, (), Dynamic>,
    pub id: VirtualColumn<Sqlite, i64>,
    pub parent: VirtualColumn<Sqlite, i64>,
    pub fullkey: VirtualColumn<Sqlite, String>,
    pub path: VirtualColumn<Sqlite, String>,
}

pub const JSON_EACH_DEFAULT: JsonEachSchema = JsonEachSchema {
    key: create_virtual_column("json_each", "key"),
    value: create_virtual_column("json_each", "value"),
    type_: create_virtual_column("json_each", "type"),
    atom: create_virtual_column("json_each", "atom"),
    id: create_virtual_column("json_each", "id"),
    parent: create_virtual_column("json_each", "parent"),
    fullkey: create_virtual_column("json_each", "fullkey"),
    path: create_virtual_column("json_each", "path"),
};

impl<T> Deref for JsonEach<T> {
    type Target = JsonEachSchema;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &JSON_EACH_DEFAULT
    }
}

impl<T> SingleArgMarker for JsonEach<T> {}

impl<'a, T, X> IntoArg<FromItem<'a, Sqlite, X>> for JsonEach<T>
    where
        T: IntoTableFuncArgs<'a> {
    fn into_arg(self) -> FromItem<'a, Sqlite, X> {
        let args = self.0.into_args();
        FromItem::from_function(ExtTableFunction::JsonEach(args.base, args.path), None)
    }
}
