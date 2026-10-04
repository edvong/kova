use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::{Function, Window};

pub struct Ntile(pub i32);

impl Ntile {
    pub fn over<'a, X>(self) -> Window<'a, Postgres, X> {
        Window::new(Function::from(ExtFunction::Ntile(self.0)))
    }
}
