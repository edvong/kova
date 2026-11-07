use crate::dsl::statement::SelectStatement;
use crate::types::{ColumnMeta, Database, Model};

pub struct ValuesClause<'a, DB: Database, T> {
    pub(crate) columns: &'static [ColumnMeta],
    pub(crate) source: ValueSource<'a, DB, T>,
}

impl<'a, DB: Database, T> ValuesClause<'a, DB, T> {
    pub(crate) fn new(columns: &'static [ColumnMeta], source: ValueSource<'a, DB, T>) -> Self {
        Self { columns, source }
    }
}

pub struct ValueSource<'a, DB: Database, T> {
    pub(crate) inner: SourceKind<'a, DB, T>,
}

pub(crate) enum SourceKind<'a, DB: Database, T> {
    Rows(SourceRows<'a, T>),
    Select(Box<SelectStatement<'a, DB>>),
}

pub(crate) enum SourceRows<'a, T> {
    Single(T),
    Many(Vec<T>),
    BorrowedSingle(&'a T),
    BorrowedMany(&'a [T]),
}

impl<'a, T> SourceRows<'a, T> {
    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Single(item) => std::slice::from_ref(item),
            Self::Many(vec) => vec.as_slice(),
            Self::BorrowedSingle(item) => std::slice::from_ref(*item),
            Self::BorrowedMany(slice) => slice,
        }
    }
}

pub trait IntoInsertSource<'a, DB: Database, T, X> {
    fn into_source(self) -> ValueSource<'a, DB, T>;
}

impl<'a, DB: Database, T: Model<DB, X>, X> IntoInsertSource<'a, DB, T, X> for T {
    #[inline(always)]
    fn into_source(self) -> ValueSource<'a, DB, T> {
        ValueSource { inner: SourceKind::Rows(SourceRows::Single(self)) }
    }
}

impl<'a, DB: Database, T: Model<DB, X>, X> IntoInsertSource<'a, DB, T, X> for Vec<T> {
    #[inline(always)]
    fn into_source(self) -> ValueSource<'a, DB, T> {
        ValueSource { inner: SourceKind::Rows(SourceRows::Many(self)) }
    }
}

impl<'a, DB: Database, T: Model<DB, X>, X> IntoInsertSource<'a, DB, T, X> for &'a T {
    #[inline(always)]
    fn into_source(self) -> ValueSource<'a, DB, T> {
        ValueSource { inner: SourceKind::Rows(SourceRows::BorrowedSingle(self)) }
    }
}

impl<'a, DB: Database, T: Model<DB, X>, X> IntoInsertSource<'a, DB, T, X> for &'a [T] {
    #[inline(always)]
    fn into_source(self) -> ValueSource<'a, DB, T> {
        ValueSource { inner: SourceKind::Rows(SourceRows::BorrowedMany(self)) }
    }
}

impl<'a, DB: Database, T: Model<DB, X>, X> IntoInsertSource<'a, DB, T, X> for &'a Vec<T> {
    #[inline(always)]
    fn into_source(self) -> ValueSource<'a, DB, T> {
        ValueSource { inner: SourceKind::Rows(SourceRows::BorrowedMany(self)) }
    }
}

impl<'a, DB: Database, T: Model<DB, X>, X> IntoInsertSource<'a, DB, T, X>
for SelectStatement<'a, DB> {
    #[inline(always)]
    fn into_source(self) -> ValueSource<'a, DB, T> {
        ValueSource { inner: SourceKind::Select(Box::new(self)) }
    }
}
