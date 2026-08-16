use crate::types::value::Value;
use kova_core::Error;
use rusqlite::RowIndex;

pub struct Row<'a> {
    pub(crate) inner: &'a rusqlite::Row<'a>,
}

impl<'a> Row<'a> {
    pub(crate) fn new(inner: &'a rusqlite::Row<'a>) -> Self {
        Self { inner }
    }

    pub fn get_value<I: RowIndex>(&self, idx: I) -> kova_core::Result<Value<'a>> {
        // get_ref() returns ValueRef < 'a > with the same lifetime as Row
        let val_ref = self.inner.get_ref(idx)
            .map_err(|e| Error::Database(Box::new(e)))?;

        Value::from_value_ref(val_ref)
    }

    // pub fn get<I: RowIndex, T: FromSql>(&self, idx: I) -> Result<T> {
    //     Ok(self.inner.get(idx).map_err(|e| Error::Database(Box::new(e)))?)
    // }
}