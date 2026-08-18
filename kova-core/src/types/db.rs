use crate::types::Mode;
use crate::types::ext::Extension;

pub trait Database: 'static + Sized {

    type Mode: Mode;

    type Connection<'a> where Self: 'a;

    type Value<'a> where Self: 'a;

    type Row<'a> where Self: 'a;

    type Extension<'a>: Extension<'a> where Self: 'a;
}