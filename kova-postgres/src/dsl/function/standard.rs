#![allow(non_snake_case)]

use crate::dsl::node::Node;
use crate::types::Postgres;
use kova_core::dsl::func::{Aggregate, Scalar};

pub type Avg<'a, T, X> = Node<kova_core::dsl::func::Avg<'a, Postgres, T, X>>;
pub type Count<'a, T, X> = Node<kova_core::dsl::func::Count<'a, Postgres, T, X>>;
pub type Max<'a, T, X> = Node<kova_core::dsl::func::Max<'a, Postgres, T, X>>;
pub type Min<'a, T, X> = Node<kova_core::dsl::func::Min<'a, Postgres, T, X>>;
pub type Sum<'a, T, X> = Node<kova_core::dsl::func::Sum<'a, Postgres, T, X>>;

pub type Abs<T> = Node<kova_core::dsl::func::Abs<T>>;
pub type Length<T> = Node<kova_core::dsl::func::Length<T>>;
pub type Lower<T> = Node<kova_core::dsl::func::Lower<T>>;
pub type Upper<T> = Node<kova_core::dsl::func::Upper<T>>;

pub type Math<'a, O, X> = Node<kova_core::dsl::Math<'a, Postgres, O, X>>;

pub const fn Avg<'a, T, X>(t: T) -> Avg<'a, T, X> {
    Node(Aggregate::new(t))
}

pub const fn Count<'a, T, X>(t: T) -> Count<'a, T, X> {
    Node(Aggregate::new(t))
}

// pub const fn CountAll<'a, X>() -> Count<'a, Wildcard, X> {
//     Count(Wildcard)
// }

pub const fn Max<'a, T, X>(t: T) -> Max<'a, T, X> {
    Node(Aggregate::new(t))
}

pub const fn Min<'a, T, X>(t: T) -> Min<'a, T, X> {
    Node(Aggregate::new(t))
}

pub const fn Sum<'a, T, X>(t: T) -> Sum<'a, T, X> {
    Node(Aggregate::new(t))
}

pub const fn Abs<T>(t: T) -> Abs<T> {
    Node(Scalar::new(t))
}

pub const fn Length<T>(t: T) -> Length<T> {
    Node(Scalar::new(t))
}

pub const fn Lower<T>(t: T) -> Lower<T> {
    Node(Scalar::new(t))
}

pub const fn Upper<T>(t: T) -> Upper<T> {
    Node(Scalar::new(t))
}

// pub(crate) const fn Math<'a>()
