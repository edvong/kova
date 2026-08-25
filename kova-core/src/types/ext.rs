pub trait Extension<'a>: Sized {
    type Column<T>;
    type Operator<'o>
    where
        Self: 'o;
    type Pagination: Default;
    type Projection;
    type Function<'t>;
    type GroupBy;
    type OrderOperand;

    // type Assignment<'t> where Self: 't;
}