use crate::types::ext::Extension;
use crate::types::{Connection, Executable, Mode};

pub trait Database: 'static + Sized {

    type Connection<'a>: Connection + Executable where Self: 'a;

    type Value<'a> where Self: 'a;

    type Row<'a> where Self: 'a;

    type Extension<'a>: Extension<'a> where Self: 'a;
}