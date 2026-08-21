pub trait Extension<'a>: Sized {
    type Column<T>;
    type Operator<'o> where Self: 'o;
    type Pagination;

    // type Assignment<'t> where Self: 't;
    // type Function;
}