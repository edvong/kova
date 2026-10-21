use crate::dsl::operand::{Dynamic, Static};

// ==========================================
// 1. Structural AST Capabilities (Parameterless)
// ==========================================
pub trait CanCompare {}
pub trait CanAlias {}
pub trait CanOrder {}

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used in a `GROUP BY` or `PARTITION BY` clause",
    label = "invalid grouping or partitioning expression"
)]
pub trait CanGroup {}

#[diagnostic::on_unimplemented(
    message = "column `{Self}` is read-only and cannot be assigned in `INSERT` / `UPDATE`",
    label = "only physical table columns (`ColumnMeta`) support assignment"
)]
pub trait CanAssignment {}

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used as an `ON CONFLICT` target",
    label = "expected a physical table column, scalar expression over physical columns, or `Constraint`"
)]
pub trait CanConflict {}

#[diagnostic::on_unimplemented(
    message = "window clause `.over(...)` is not available on `{Self}`",
    label = "only aggregate and window functions support `.over(...)`"
)]
pub trait CanWindow {}

#[diagnostic::on_unimplemented(
    message = "`.distinct()` modifier is not available on `{Self}`",
    label = "only aggregate functions support `.distinct()`"
)]
pub trait CanDistinct {}

#[diagnostic::on_unimplemented(
    message = "aggregate `.filter(WHERE ...)` modifier is not available on `{Self}`",
    label = "only aggregate functions support `.filter(...)`"
)]
pub trait CanFilter {}

pub trait CanOrderBy {}

// ==========================================
// 2. Data-Type Domain Capabilities (Codec-Aware <X>)
// ==========================================

#[diagnostic::on_unimplemented(
    message = "textual capability (`CanString<{X}>`) is not satisfied for `{Self}`",
    label = "expected a textual column or string expression",
    note = "if `{Self}` is a custom textual type, implement `CanString<{X}>` for it."
)]
pub trait CanString<X> {} // like, not_like, Lower, Upper, Length

#[diagnostic::on_unimplemented(
    message = "numeric capability (`CanMath<{X}>`) is not satisfied for `{Self}`",
    label = "expected a numeric column or numeric expression",
    note = "if `{Self}` is a custom numeric type (e.g., `Decimal`), implement `CanMath<{X}>` for it."
)]
pub trait CanMath<X> {} // add, sub, mul, div, increment, decrement, Abs, Sum, Avg

impl<X> CanMath<X> for Dynamic {}
impl<X> CanString<X> for Dynamic {}

impl<T: ?Sized + CanMath<X>, X> CanMath<X> for Static<T> {}
impl<'a, T: ?Sized + CanMath<X>, X> CanMath<X> for &'a T {}
impl<T: ?Sized + CanMath<X>, X> CanMath<X> for Box<T> {}
impl<T: CanMath<X>, X> CanMath<X> for Option<T> {}

impl<T: ?Sized + CanString<X>, X> CanString<X> for Static<T> {}
impl<'a, T: ?Sized + CanString<X>, X> CanString<X> for &'a T {}
impl<T: ?Sized + CanString<X>, X> CanString<X> for Box<T> {}
impl<T: CanString<X>, X> CanString<X> for Option<T> {}

macro_rules! impl_can_math {
    ($($ty:ty),+) => {$(
            impl<X> CanMath<X> for $ty {}
        )+
    };
}

impl_can_math!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl<X> CanString<X> for String {}
impl<X> CanString<X> for str {}
impl<X> CanString<X> for char {}
