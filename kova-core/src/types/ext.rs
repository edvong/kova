pub trait Extension<'a>: Sized {
    type Operator<'o> where Self: 'o;

    // Expression Extension
    type Function<'t> where Self: 't;

    type TableFunction<'t> where Self: 't;

    type OrderOperand;

    type Assignment<'t> where Self: 't;

    // Clause Extension
    type GroupBy;
    type Locking: Default;
}
