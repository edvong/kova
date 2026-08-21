use crate::types::ext::Extension;
use crate::types::{Connection, Executable, Query};

pub trait Database: 'static + Sized {
    type Value<'a> where Self: 'a;

    type Row<'a> where Self: 'a;

    type Connection<'a>: Connection + Executable where Self: 'a;

    type Query<'a>: Query<'a, Self> where Self: 'a;

    type Extension<'a>: Extension<'a> where Self: 'a;
}