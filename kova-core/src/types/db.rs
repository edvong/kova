use crate::types::ext::Extension;

pub trait Database: 'static + Sized {

    type Value<'a> where Self: 'a;

    type Row<'r>: 'r;

    type Extension<'e>: Extension<'e>;
}