mod assignment;
mod bind;
mod combine;
pub(crate) mod conflict;
mod distinct;
mod group_by;
mod having;
pub(crate) mod insert;
pub(crate) mod join;
mod locking;
pub(crate) mod order;
mod pagination;
mod projection;
pub(crate) mod table;
mod r#where;
mod with;

pub use {
    bind::Bind,
    combine::{SetOperation, SetOperator},
    conflict::{
        Conflict, ConflictAction, ExtConflictTarget, OnConflictClause, IntoConflict,
        IntoConflictTarget
    },
    group_by::{GroupBy, IntoGroupBy},
    insert::{IntoInsertSource, ValueSource},
    join::{IntoJoinConstraint, Join, JoinClause, JoinConstraint, JoinType},
    locking::ExtLocking,
    order::{Direction, IntoOrdering, NullsOrder, OrderByClause, Ordering},
    pagination::PaginationClause,
    projection::{IntoProjection, Projection},
    table::FromItem,
};

pub(crate) use {
    assignment::AssignmentClause,
    bind::BindClause,
    combine::CombineClause,
    distinct::DistinctClause,
    group_by::GroupByClause,
    having::HavingClause,
    insert::ValuesClause,
    locking::LockingClause,
    projection::ProjectionClause,
    r#where::WhereClause,
    table::TableClause,
    with::{CteNode, WithClause},
};

pub trait ExtensionClause<E> {
    fn ext(&self) -> &E;
    fn ext_mut(&mut self) -> &mut E;
}
