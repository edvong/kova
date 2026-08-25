use crate::dsl::arg::{IntoArg, IsSingleArg};
use crate::dsl::function::Function;
use crate::dsl::{ExtensionClause, Column, Component, Condition, IntoArgs};
use crate::types::{ColumnMeta, Database, Extension};

pub struct OrderByClause<'a, DB: Database> {
    items: Vec<Ordering<'a, DB>>,
}

impl<'a, DB: Database> OrderByClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn from(ordering: Vec<Ordering<'a, DB>>) -> Self {
        Self { items: ordering }
    }

    pub fn set<I>(&mut self, args: I)
    where
        I: IntoArgs<Ordering<'a, DB>>
    {
        self.items.clear();
        self.extend(args);
    }

    pub fn extend<I>(&mut self, args: I)
    where
        I: IntoArgs<Ordering<'a, DB>>
    {
        self.items.reserve(args.size_hint());
        args.append_args(&mut self.items);
    }
}

pub struct Ordering<'a, DB: Database> {
    pub(crate) operand: OrderOperand<'a, DB>,
    pub(crate) direction: Direction,
    pub(crate) nulls: Option<NullsOrder>,
}

impl<'a, DB: Database> Ordering<'a, DB> {
    pub fn nulls_first(mut self) -> Self {
        self.nulls = Some(NullsOrder::First);
        self
    }

    pub fn nulls_last(mut self) -> Self {
        self.nulls = Some(NullsOrder::Last);
        self
    }
}

pub type OrderOperand<'a, DB> = Component<
    StandardOperand<'a, DB>,
    <<DB as Database>::Extension<'a> as Extension<'a>>::OrderOperand
>;

pub enum StandardOperand<'a, DB: Database> {
    Column(ColumnMeta),
    Function(Function<'a, DB>),
    Condition(Condition<'a, DB>),
}

pub enum Direction {
    Asc,
    Desc,
}

pub enum NullsOrder {
    First,
    Last,
}

impl<'a, DB: Database> IsSingleArg for Ordering<'a, DB> {}

impl<'a, T, DB: Database> IntoArg<Ordering<'a, DB>> for Column<'a, T, DB> {
    fn into_arg(self) -> Ordering<'a, DB> {
        self.asc()
    }
}

impl<'a, DB: Database> IntoArg<Ordering<'a, DB>> for Function<'a, DB> {
    fn into_arg(self) -> Ordering<'a, DB> {
        self.asc()
    }
}

impl<'a, DB: Database> IntoArg<Ordering<'a, DB>> for Condition<'a, DB> {
    fn into_arg(self) -> Ordering<'a, DB> {
        self.asc()
    }
}


impl<'a, DB: Database> ExtensionClause for OrderByClause<'a, DB> {
    type S = ();
    type E = ();

    fn component(&self) -> &Option<Component<Self::S, Self::E>> {
        todo!()
    }

    fn component_mut(&mut self) -> &mut Option<Component<Self::S, Self::E>> {
        todo!()
    }
}