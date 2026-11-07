use crate::Result;
use crate::dsl::{ArgEncode, EraseX, IntoArg};
use crate::types::{Database, Encode};
use std::marker::PhantomData;

pub struct BindClause<'a, DB: Database> {
    pub(crate) params: Vec<Bind<'a, DB, ()>>,
}

pub struct Bind<'a, DB: Database, X> {
    value: Result<DB::Value<'a>>,
    _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> Bind<'a, DB, X> {
    #[inline(always)]
    pub fn new(value: Result<DB::Value<'a>>) -> Self {
        Self { value, _marker: PhantomData }
    }
}


impl<'a, DB: Database, X> EraseX for Bind<'a, DB, X> {
    type Erased = Bind<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Bind::new(self.value)
    }
}

impl<'a, DB: Database> BindClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { params: Vec::new() }
    }

    pub fn set(&mut self, args: Vec<Bind<'a, DB, ()>>) {
        self.params = args;
    }

    pub fn to_values(self) -> Result<Vec<DB::Value<'a>>> {
        let mut values = Vec::with_capacity(self.params.len());
        for p in self.params {
            values.push(p.value?);
        }
        Ok(values)
    }
}

impl<'a, DB: Database, T, X> IntoArg<Bind<'a, DB, X>, ArgEncode> for T
    where
        DB: Encode<'a, T, X> {
    fn into_arg(self) -> Bind<'a, DB, X> {
        Bind::new(DB::encode(self))
    }
}
