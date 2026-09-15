pub trait Extension<'a>: Sized {
    type Column<'c, T>
    where
        T: 'c;

    // Self: 'c,

    type Operator<'o>
    where
        Self: 'o;

    // Expression Extension
    type Function<'t>
    where
        Self: 't;

    type OrderOperand;

    type Assignment<'t>
    where
        Self: 't;

    // Clause Extension
    type GroupBy;
    type Locking;
    type Pagination;
    type OnConflict<'c>;
}
