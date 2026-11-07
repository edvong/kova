
pub trait Extension: Sized {
    type Operator<'a> where Self: 'a;

    type Function<'a> where Self: 'a;

    type TableFunction<'a> where Self: 'a;

    type Assignment<'a> where Self: 'a;

    type ConflictTarget;

    type Locking: Default;
}
