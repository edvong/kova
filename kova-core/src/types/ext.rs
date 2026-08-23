pub trait Extension<'a>: Sized {
    type Column<T>;
    type Operator<'o>
    where
        Self: 'o;
    type Pagination;
    type Projection;
    type Function<'t>;

    // type Assignment<'t> where Self: 't;
}