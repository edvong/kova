use crate::dsl::Pagination;
use crate::types::Postgres;
use kova_core::dsl::{
    Condition, EraseX, GroupBy, IntoAlias, IntoArg, IntoArgs, IntoOperand, IsAst, Join, JoinType,
    Operand, Ordering, Projection, ProjectionExpr, SingleArgMarker,
};
use kova_core::statement::SelectStatement;
use kova_core::types::Model;
use std::marker::PhantomData;

pub struct Entry;
pub struct Chain<X>(PhantomData<X>);

pub struct Subselect<'a, T: Model, State = Entry> {
    pub(crate) statement: SelectStatement<'a, T::DB>,
    pub(crate) _marker: PhantomData<(T, State)>,
}

impl<'a, T: Model<DB = Postgres>> Subselect<'a, T, Entry> {
    pub fn columns<C, X>(cols: C) -> Subselect<'a, T, Chain<X>>
    where
        C: IntoArgs<Projection<'a, Postgres, X>>,
    {
        let mut statement = SelectStatement::new(T::TABLE_NAME);
        statement.columns(cols.into_mapped_args(|p| p.erase_x()));
        Subselect {
            statement,
            _marker: PhantomData,
        }
    }
}


impl<'a, T: Model<DB = Postgres>, X> Subselect<'a, T, Chain<X>> {
    pub fn filter(mut self, cond: Condition<'a, T::DB, X>) -> Self {
        self.statement.r#where(cond.erase_x());
        self
    }

    pub fn inner_join<M: Model<DB = T::DB>>(mut self, cond: Condition<'a, T::DB, X>) -> Self {
        self.statement.join(Join {
            typ: JoinType::Inner,
            target_table: M::TABLE_NAME,
            on: cond.erase_x(),
        });
        self
    }

    pub fn left_join<M: Model<DB = T::DB>>(mut self, cond: Condition<'a, T::DB, X>) -> Self {
        self.statement.join(Join {
            typ: JoinType::Left,
            target_table: M::TABLE_NAME,
            on: cond.erase_x(),
        });
        self
    }

    pub fn group_by<C>(mut self, cols: C) -> Self
    where
        C: IntoArgs<GroupBy<'a, T::DB, X>>,
    {
        self.statement
            .group_by(cols.into_mapped_args(|g| g.erase_x()));
        self
    }

    pub fn having(mut self, cond: Condition<'a, T::DB, X>) -> Self {
        self.statement.having(cond.erase_x());
        self
    }

    pub fn order_by<C>(mut self, cols: C) -> Self
    where
        C: IntoArgs<Ordering<'a, T::DB, X>>,
    {
        self.statement
            .order_by(cols.into_mapped_args(|o| o.erase_x()));
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        let ext = self
            .statement
            .pagination_ext_mut_or_with(Pagination::default);
        ext.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        let ext = self
            .statement
            .pagination_ext_mut_or_with(Pagination::default);
        ext.offset = Some(offset);
        self
    }

    pub fn alias<I: IntoAlias<A>, A>(self, alias: I) -> Projection<'a, T::DB, X> {
        let mut proj: Projection<'a, T::DB, X> = self.into_arg();
        proj.alias(alias.into_identifier());
        proj
    }
}


impl<'a, T: Model, X> SingleArgMarker for Subselect<'a, T, Chain<X>> {}

impl<'a, T: Model<DB = Postgres>, X> IntoArg<Projection<'a, T::DB, X>>
    for Subselect<'a, T, Chain<X>>
{
    fn into_arg(self) -> Projection<'a, Postgres, X> {
        Projection::new(ProjectionExpr::Subselect(Box::new(self.statement)), None)
    }
}

impl<'a, T: Model<DB = Postgres>, X> IntoOperand<'a, T, Postgres, IsAst, X>
    for Subselect<'a, T, Chain<X>>
{
    fn into_operand(self) -> Operand<'a, Postgres> {
        Operand::Subselect(Box::new(self.statement))
    }
}
