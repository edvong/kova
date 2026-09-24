use crate::dsl::{
    CombineClause, Condition, CteNode, DistinctClause, EraseX, ExtLocking, ExtensionClause,
    GroupBy, GroupByClause, HavingClause, IntoArgs, IntoJoinConstraint, Join, JoinClause,
    JoinConstraint, JoinType, LockingClause, OrderByClause, Ordering, PaginationClause, Projection,
    ProjectionClause, SetOperator, TableClause, TableItem, WhereClause, WithClause,
};
use crate::render::RenderData;
use crate::types::{Cte, Database, Relation};

pub struct SelectStatement<'a, DB: Database> {
    with: WithClause<'a, DB>,
    from: TableClause<'a, DB>,
    distinct: DistinctClause,
    projection: ProjectionClause<'a, DB>,
    join: JoinClause<'a, DB>,
    r#where: WhereClause<'a, DB>,
    group_by: GroupByClause<'a, DB>,
    having: HavingClause<'a, DB>,
    order_by: OrderByClause<'a, DB>,
    pagination: PaginationClause,
    locking: LockingClause<'a, DB>,
    combine: CombineClause<'a, DB>,
}

pub trait IntoSelectStatement<'a, DB: Database, X> {
    fn into_select_statement(self) -> SelectStatement<'a, DB>;
}

// It is also implemented for SelectStatement itself to ensure the completeness of polymorphism.
impl<'a, DB: Database, X> IntoSelectStatement<'a, DB, X> for SelectStatement<'a, DB> {
    #[inline(always)]
    fn into_select_statement(self) -> SelectStatement<'a, DB> {
        self
    }
}

impl<'a, DB> SelectStatement<'a, DB>
    where
        DB: Database {
    pub fn new(table: &'static str) -> Self {
        Self {
            with: WithClause::new(),
            from: TableClause::new_with(table),
            distinct: DistinctClause::new(),
            projection: ProjectionClause::new(),
            join: JoinClause::new(),
            r#where: WhereClause::new(),
            group_by: GroupByClause::new(),
            having: HavingClause::new(),
            order_by: OrderByClause::new(),
            pagination: PaginationClause::new(),
            locking: LockingClause::new(),
            combine: CombineClause::new(),
        }
    }

    pub fn build(&self) -> RenderData<'a, DB> {
        let data = RenderData::new();
        // let mut ctx = SelectContext::new(&mut data);
        // self.render(&mut ctx);
        data
    }

    pub fn from<A: IntoArgs<TableItem<'a, DB>>>(&mut self, items: A) {
        self.from.extend(items.into_args());
    }

    pub fn with<C: Cte>(&mut self, statement: SelectStatement<'a, DB>) {
        self.with.add(CteNode { name: C::CTE_NAME, statement: Box::new(statement) });
    }

    pub fn distinct(&mut self) {
        self.distinct.set(true);
    }

    pub fn columns<A, X>(&mut self, cols: A)
        where
            A: IntoArgs<Projection<'a, DB, X>> {
        self.projection.set(cols.into_mapped_args(|p| p.erase_x()));
    }

    pub fn join<R: Relation, X>(
        &mut self, typ: JoinType, constraint: impl IntoJoinConstraint<'a, DB, X>,
    ) {
        let join =
            Join { typ, target_table: R::RELATION_NAME, constraint: constraint.into_constraint() };
        self.join.add(join)
    }

    pub fn cross_join<R: Relation>(&mut self) {
        let join = Join {
            typ: JoinType::Cross,
            target_table: R::RELATION_NAME,
            constraint: JoinConstraint::None,
        };
        self.join.add(join);
    }

    pub fn r#where<X>(&mut self, cond: Condition<'a, DB, X>) {
        self.r#where.set(cond.erase_x())
    }

    pub fn group_by<A, X>(&mut self, cols: A)
        where
            A: IntoArgs<GroupBy<'a, DB, X>> {
        self.group_by.set(cols.into_mapped_args(|g| g.erase_x()))
    }

    pub fn having<X>(&mut self, cond: Condition<'a, DB, X>) {
        self.having.set(cond.erase_x());
    }

    pub fn order_by<A, X>(&mut self, cols: A)
        where
            A: IntoArgs<Ordering<'a, DB, X>> {
        self.order_by.set(cols.into_mapped_args(|o| o.erase_x()));
    }

    pub fn limit(&mut self, limit: usize) {
        self.pagination.limit(limit);
    }

    pub fn offset(&mut self, offset: usize) {
        self.pagination.offset(offset);
    }

    pub fn locking_mut(&mut self) -> &mut ExtLocking<'a, DB> {
        self.locking.ext_mut()
    }

    pub fn combine(&mut self, operator: SetOperator, statement: SelectStatement<'a, DB>) {
        self.combine.add(operator, statement);
    }
}
