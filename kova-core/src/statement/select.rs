use crate::dsl::{ExtensionClause, Condition, ExtPagination, GroupBy, GroupByClause, HavingClause, IntoArgs, Join, JoinClause, OrderByClause, Ordering, PaginationClause, Projection, ProjectionClause, Where};
use crate::render::{Render, RenderData, SelectContext};
use crate::types::{Database, Model};
use std::marker::PhantomData;

pub struct SelectStatement<'a, T, DB: Database> {
    projection: ProjectionClause<'a, DB>,
    join: JoinClause<'a, DB>,
    r#where: Where<'a, DB>,
    group_by: GroupByClause<'a, DB>,
    having: HavingClause<'a, DB>,
    order_by: OrderByClause<'a, DB>,
    pagination: PaginationClause<'a, DB>,
    _marker: PhantomData<T>,
}

impl<'a, T, DB> SelectStatement<'a, T, DB>
where
    T: Model<DB = DB>,
    DB: Database,
{
    pub fn new() -> Self {
        Self {
            projection: ProjectionClause::new(),
            join: JoinClause::new(),
            r#where: Where::new(),
            group_by: GroupByClause::new(),
            having: HavingClause::new(),
            order_by: OrderByClause::new(),
            pagination: PaginationClause::new(),
            _marker: PhantomData,
        }
    }

    pub fn build(&self) -> RenderData<'a, DB> {
        let mut data = RenderData::new();
        let mut ctx = SelectContext::new(&mut data);
        self.render(&mut ctx);
        data
    }

    pub fn columns<C>(&mut self, cols: C)
    where
        C: IntoArgs<Projection<'a, DB>>,
    {
        self.projection.set(cols);
    }

    pub fn join(&mut self, join: Join<'a, DB>) {
        self.join.add(join)
    }

    pub fn r#where(&mut self, cond: Condition<'a, DB>) {
        self.r#where.set(cond)
    }

    pub fn having(&mut self, cond: Condition<'a, DB>) {
        self.having.set(cond);
    }

    pub fn group_by<C>(&mut self, cols: C) where C: IntoArgs<GroupBy<'a, DB>> {
        self.group_by.set(cols)
    }

    pub fn order_by<O>(&mut self, order: O)
    where
        O: IntoArgs<Ordering<'a, DB>>
    {
        self.order_by.set(order);
    }

    pub fn pagination_ext_mut(&mut self) -> Option<&mut ExtPagination<'a, DB>> {
        self.pagination.ext_mut_or_default()
    }

}
