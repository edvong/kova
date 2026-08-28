pub trait Extension<'a>: Sized {
    type Column<T>;

    type Operator<'o>
    where
        Self: 'o;

    // Expression Extension
    type Function<'t>;
    type OrderOperand;

    // Clause Extension
    type GroupBy;
    type Locking;
    type Pagination;



    // type Assignment<'t> where Self: 't;
}