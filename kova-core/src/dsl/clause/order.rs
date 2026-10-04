use crate::dsl::arg::SingleArgMarker;
use crate::dsl::{Condition, EraseX, Operand};
use crate::types::Database;
use std::marker::PhantomData;

pub struct OrderByClause<'a, DB: Database> {
    items: Vec<Ordering<'a, DB, ()>>,
}

impl<'a, DB: Database> OrderByClause<'a, DB> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn from(ordering: Vec<Ordering<'a, DB, ()>>) -> Self {
        Self { items: ordering }
    }

    pub fn set(&mut self, args: Vec<Ordering<'a, DB, ()>>) {
        self.items = args;
    }
}

pub struct Ordering<'a, DB: Database, X> {
    operand: OrderOperand<'a, DB>,
    direction: Option<Direction>,
    nulls: Option<NullsOrder>,
    _marker: PhantomData<X>,
}

enum OrderOperand<'a, DB: Database> {
    Operand(Operand<'a, DB>),
    Condition(Condition<'a, DB, ()>),
}

impl<'a, DB: Database, X> Ordering<'a, DB, X> {
    pub fn from_operand(operand: Operand<'a, DB>) -> Self {
        Self { operand: OrderOperand::Operand(operand), direction: None, nulls: None, _marker: PhantomData }
    }

    pub fn from_condition(condition: Condition<'a, DB, ()>) -> Self {
        Self { operand: OrderOperand::Condition(condition), direction: None, nulls: None, _marker: PhantomData }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn nulls(mut self, nulls_order: NullsOrder) -> Self {
        self.nulls = Some(nulls_order);
        self
    }
}

pub trait IntoOrdering<'a, DB: Database, X> {
    fn into_ordering(self) -> Ordering<'a, DB, X>;
}

// #[deprecated]
// pub trait BuildOrdering<'a, DB: Database, X> {
//     #[deprecated]
//     fn build_ordering(self, direction: Direction) -> Ordering<'a, DB, X>;
// }

impl<'a, DB: Database, X> EraseX for Ordering<'a, DB, X> {
    type Erased = Ordering<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Ordering {
            operand: self.operand,
            direction: self.direction,
            nulls: self.nulls,
            _marker: PhantomData,
        }
    }
}

impl<'a, DB: Database, X> Ordering<'a, DB, X> {
    pub fn nulls_first(mut self) -> Self {
        self.nulls = Some(NullsOrder::First);
        self
    }

    pub fn nulls_last(mut self) -> Self {
        self.nulls = Some(NullsOrder::Last);
        self
    }
}


// pub struct OrderOperand<'a, DB: Database> {
//     pub(crate) inner: Component<
//         StandardOperand<'a, DB>,
//         <<DB as Database>::Extension<'a> as Extension<'a>>::OrderOperand,
//     >,
// }

// pub type ExtOrderOperand<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::OrderOperand;
//
// impl<'a, DB: Database> WithComponent for OrderOperand<'a, DB> {
//     type Standard = StandardOperand<'a, DB>;
//     type Extension = ExtOrderOperand<'a, DB>;
//
//     fn from_std(std: Self::Standard) -> Self {
//         Self { inner: Component::standard(std) }
//     }
//
//     fn from_ext(ext: Self::Extension) -> Self {
//         Self { inner: Component::extension(ext) }
//     }
// }
//
// pub enum StandardOperand<'a, DB: Database> {
//     Column(IdentifierMeta),
//     Function(Function<'a, DB>),
//     Condition(Condition<'a, DB, ()>),
// }

pub enum Direction {
    Asc,
    Desc,
}

pub enum NullsOrder {
    First,
    Last,
}

impl<'a, DB: Database, X> SingleArgMarker for Ordering<'a, DB, X> {}

// impl<'a, T, DB: Database, X> IntoArg<Ordering<'a, DB, X>> for Column<T, DB> {
//     fn into_arg(self) -> Ordering<'a, DB, X> {
//         self.build_ordering(Direction::Asc)
//     }
// }

// impl<'a, DB: Database, X> IntoArg<Ordering<'a, DB, X>> for Condition<'a, DB, X> {
//     fn into_arg(self) -> Ordering<'a, DB, X> {
//         self.build_ordering(Direction::Asc)
//     }
// }

// impl<'a, DB: Database, X> IntoArg<Ordering<'a, DB, X>> for Function<'a, DB> {
//     fn into_arg(self) -> Ordering<'a, DB, X> {
//         // self.asc::<X>()
//         // self.build_ordering(Some(Direction::Asc))
//         todo!()
//     }
// }
