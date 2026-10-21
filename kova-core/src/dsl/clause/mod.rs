mod assignment;
mod bind;
mod combine;
mod conflict;
mod distinct;
mod group_by;
mod having;
mod insert;
mod join;
mod locking;
mod order;
mod pagination;
mod projection;
mod table;
mod r#where;
mod with;

pub use {
    bind::Bind,
    combine::{SetOperation, SetOperator},
    conflict::{Conflict, IntoConflict},
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
    conflict::OnConflictClause,
    distinct::DistinctClause,
    group_by::GroupByClause,
    having::HavingClause,
    insert::ValuesClause,
    locking::LockingClause,
    projection::ProjectionClause,
    table::TableClause,
    r#where::WhereClause,
    with::{CteNode, WithClause},
};

pub trait ExtensionClause<E> {
    fn ext(&self) -> &E;
    fn ext_mut(&mut self) -> &mut E;
}
