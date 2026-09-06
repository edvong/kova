use crate::dsl::{
    Condition, DistinctClause, ExtLocking, ExtPagination, ExtensionClause, GroupBy, GroupByClause,
    HavingClause, Join, JoinClause, LockingClause, OrderByClause, Ordering, PaginationClause,
    Projection, ProjectionClause, WhereClause,
};
use crate::render::RenderData;
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct SelectStatement<'a, DB: Database, T> {
    distinct: DistinctClause,
    projection: ProjectionClause<'a, DB>,
    join: JoinClause<'a, DB>,
    r#where: WhereClause<'a, DB>,
    group_by: GroupByClause<'a, DB>,
    having: HavingClause<'a, DB>,
    order_by: OrderByClause<'a, DB>,
    pagination: PaginationClause<'a, DB>,
    locking: LockingClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, DB, T> SelectStatement<'a, DB, T>
where
    T: Model<DB = DB>,
    DB: Database,
{
    pub fn new() -> Self {
        Self {
            distinct: DistinctClause::new(),
            projection: ProjectionClause::new(),
            join: JoinClause::new(),
            r#where: WhereClause::new(),
            group_by: GroupByClause::new(),
            having: HavingClause::new(),
            order_by: OrderByClause::new(),
            pagination: PaginationClause::new(),
            locking: LockingClause::new(),
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> RenderData<'a, DB> {
        let data = RenderData::new();
        // let mut ctx = SelectContext::new(&mut data);
        // self.render(&mut ctx);
        data
    }

    pub fn distinct(&mut self) {
        self.distinct.set(true);
    }

    pub fn columns(&mut self, args: Vec<Projection<'a, DB, ()>>) {
        self.projection.set(args);
    }

    pub fn join(&mut self, join: Join<'a, DB>) {
        self.join.add(join)
    }

    pub fn r#where(&mut self, cond: Condition<'a, DB, ()>) {
        self.r#where.set(cond)
    }

    pub fn having(&mut self, cond: Condition<'a, DB, ()>) {
        self.having.set(cond);
    }

    pub fn group_by(&mut self, args: Vec<GroupBy<'a, DB, ()>>) {
        self.group_by.set(args)
    }

    pub fn order_by(&mut self, orders: Vec<Ordering<'a, DB, ()>>) {
        self.order_by.set(orders);
    }

    pub fn pagination_ext_mut_or_with<F>(&mut self, f: F) -> &mut ExtPagination<'a, DB>
    where
        F: FnOnce() -> ExtPagination<'a, DB>,
    {
        self.pagination.ext_mut_or_with(f)
    }

    pub fn locking_ext_mut_or_with<F>(&mut self, f: F) -> &mut ExtLocking<'a, DB>
    where
        F: FnOnce() -> ExtLocking<'a, DB>,
    {
        self.locking.ext_mut_or_with(f)
    }
}
