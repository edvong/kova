mod assignment;
mod bind;
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
mod r#where;
mod with;

pub use {
    bind::Bind,
    conflict::ExtOnConflict,
    group_by::{GroupBy, StandardGroupBy},
    join::{Join, JoinClause, JoinType},
    locking::ExtLocking,
    order::{
        BuildOrdering, Direction, NullsOrder, OrderByClause, OrderOperand, Ordering,
        StandardOperand,
    },
    pagination::PaginationClause,
    projection::{BuildProjection, Projection, ProjectionExpr},
};

pub(crate) use {
    assignment::AssignmentClause,
    bind::BindClause,
    conflict::OnConflictClause,
    distinct::DistinctClause,
    group_by::GroupByClause,
    having::HavingClause,
    insert::ValuesClause,
    locking::LockingClause,
    projection::ProjectionClause,
    r#where::WhereClause,
    with::{CteNode, WithClause},
};

pub trait ExtensionClause<E> {
    fn ext(&self) -> &E;
    fn ext_mut(&mut self) -> &mut E;
}
