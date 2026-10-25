use crate::types::ext::Extension;
use crate::types::{Connection, Executable, Query, Row, Value};

pub trait Database: 'static + Sized {
    const MAX_PARAMS: usize;

    type Value<'a>: Value where Self: 'a;

    type Row<'a>: Row<'a, Self> where Self: 'a;

    type Connection: Connection + Executable;

    type Query: for<'a> Query<'a, Self>;

    type Extension: Extension;
}
