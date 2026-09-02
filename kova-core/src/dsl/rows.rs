pub trait IntoRows<'a, T, Marker> {
    fn into_rows(self) -> InsertRows<'a, T>;
}

pub enum InsertRows<'a, T> {
    Single(T),
    Many(Vec<T>),
    BorrowedSingle(&'a T),
    BorrowedMany(&'a [T]),
}

impl<'a, T> InsertRows<'a, T> {
    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Single(item) => std::slice::from_ref(item),
            Self::Many(vec) => vec.as_slice(),
            Self::BorrowedSingle(item) => std::slice::from_ref(*item),
            Self::BorrowedMany(slice) => slice,
        }
    }
}

pub struct SingleRow;
pub struct ManyRows;
pub struct BorrowedSingleRow;
pub struct BorrowedManyRows;

impl<'a, T> IntoRows<'a, T, SingleRow> for T {
    fn into_rows(self) -> InsertRows<'a, T> {
        InsertRows::Single(self)
    }
}

impl<'a, T> IntoRows<'a, T, ManyRows> for Vec<T> {
    fn into_rows(self) -> InsertRows<'a, T> {
        InsertRows::Many(self)
    }
}

impl<'a, T> IntoRows<'a, T, BorrowedSingleRow> for &'a T {
    fn into_rows(self) -> InsertRows<'a, T> {
        InsertRows::BorrowedSingle(self)
    }
}

impl<'a, T> IntoRows<'a, T, BorrowedManyRows> for &'a [T] {
    fn into_rows(self) -> InsertRows<'a, T> {
        InsertRows::BorrowedMany(self)
    }
}

impl<'a, T> IntoRows<'a, T, BorrowedManyRows> for &'a Vec<T> {
    fn into_rows(self) -> InsertRows<'a, T> {
        InsertRows::BorrowedMany(self.as_slice())
    }
}
