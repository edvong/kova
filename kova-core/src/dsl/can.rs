// eq, ne, gt, ge, lt, le, is_null, is_not_null, between, not_between, in_list, not_in_list
pub trait CanCompare {}
pub trait CanString {} // like, not_like
pub trait CanMath {} // add, sub, mul, div
pub trait CanAlias {} // alias
pub trait CanOrder {} // asc, desc, (nulls_first, nulls_last)?
pub trait CanGroup {}
pub trait CanAssignment {}
pub trait CanWindow {} // over, distinct
pub trait CanDistinct {}
pub trait CanFilter {} // filter (For AGG_FUNC() FILTER (WHERE ...)
pub trait CanOrderBy {}
// pub trait Textual {}
// pub trait Numeric {}
// pub trait Windowable {}
