use crate::dsl::ExtColumn;
use crate::dsl::function::ExtTableFunction;
use crate::dsl::function::from::IntoTableFuncArgs;
use crate::types::Sqlite;
use kova_core::dsl::{Column, IntoArg, SingleArgMarker, Table};
use kova_core::types::IdentifierMeta;
use std::ops::Deref;

pub struct JsonEach<T = ()>(pub T);

#[allow(non_upper_case_globals)]
impl JsonEach<()> {
    pub const key: Column<'static, String, Sqlite> = JSON_EACH_DEFAULT.key;
    pub const value: Column<'static, String, Sqlite> = JSON_EACH_DEFAULT.value;
    pub const type_: Column<'static, String, Sqlite> = JSON_EACH_DEFAULT.type_;
    pub const atom: Column<'static, String, Sqlite> = JSON_EACH_DEFAULT.atom;
    pub const id: Column<'static, i32, Sqlite> = JSON_EACH_DEFAULT.id;
    pub const parent: Column<'static, i32, Sqlite> = JSON_EACH_DEFAULT.parent;
    pub const fullkey: Column<'static, String, Sqlite> = JSON_EACH_DEFAULT.fullkey;
    pub const path: Column<'static, String, Sqlite> = JSON_EACH_DEFAULT.path;
}

pub struct JsonEachSchema {
    pub key: Column<'static, String, Sqlite>,
    pub value: Column<'static, String, Sqlite>,
    pub type_: Column<'static, String, Sqlite>,
    pub atom: Column<'static, String, Sqlite>,
    pub id: Column<'static, i32, Sqlite>,
    pub parent: Column<'static, i32, Sqlite>,
    pub fullkey: Column<'static, String, Sqlite>,
    pub path: Column<'static, String, Sqlite>,
}

pub const JSON_EACH_DEFAULT: JsonEachSchema = JsonEachSchema {
    key: ExtColumn::create(IdentifierMeta::new("json_each", "key")),
    value: ExtColumn::create(IdentifierMeta::new("json_each", "value")),
    type_: ExtColumn::create(IdentifierMeta::new("json_each", "type")),
    atom: ExtColumn::create(IdentifierMeta::new("json_each", "atom")),
    id: ExtColumn::create(IdentifierMeta::new("json_each", "id")),
    parent: ExtColumn::create(IdentifierMeta::new("json_each", "parent")),
    fullkey: ExtColumn::create(IdentifierMeta::new("json_each", "fullkey")),
    path: ExtColumn::create(IdentifierMeta::new("json_each", "path")),
};

impl<T> Deref for JsonEach<T> {
    type Target = JsonEachSchema;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &JSON_EACH_DEFAULT
    }
}

impl<T> SingleArgMarker for JsonEach<T> {}

impl<'a, T, X> IntoArg<Table<'a, Sqlite, X>> for JsonEach<T>
    where
        T: IntoTableFuncArgs<'a> {
    fn into_arg(self) -> Table<'a, Sqlite, X> {
        let args = self.0.into_args();
        Table::from_function(ExtTableFunction::JsonEach(args.base, args.path), None)
    }
}
