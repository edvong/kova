use crate::dsl::{EraseX, IntoArg, IntoIdentifier, IntoProjection, Operand, Ordering, Projection};
use crate::types::Database;
use std::marker::PhantomData;

pub struct Window<'a, DB: Database, X> {
    pub(crate) partition_by: Vec<PartitionBy<'a, DB, ()>>,
    pub(crate) order_by: Vec<Ordering<'a, DB, ()>>,
    pub(crate) frame: Option<WindowFrame>,
    _marker: PhantomData<X>,
}

#[allow(non_snake_case)]
pub const fn Window<'a, DB: Database, X>() -> Window<'a, DB, X> {
    Window { partition_by: Vec::new(), order_by: Vec::new(), frame: None, _marker: PhantomData }
}

impl<'a, DB: Database, X> EraseX for Window<'a, DB, X> {
    type Erased = Window<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Window {
            partition_by: self.partition_by,
            order_by: self.order_by,
            frame: self.frame,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WindowFrame {
    pub typ: FrameType,
    pub start: FrameBound,
    pub end: Option<FrameBound>,
}

#[derive(Debug, Copy, Clone)]
pub enum FrameType {
    Rows,
    Range,
    Groups,
}

#[derive(Debug, Copy, Clone)]
pub enum FrameBound {
    UnboundedPreceding,
    Preceding(usize),
    CurrentRow,
    Following(usize),
    UnboundedFollowing,
}

impl<'a, DB: Database, X> Window<'a, DB, X> {
    pub fn partition_by<C, M>(mut self, cols: C) -> Self
        where
            C: IntoArg<PartitionBy<'a, DB, X>, M> {
        self.partition_by = cols.into_args_erased();
        self
    }

    pub fn rows(mut self, start: FrameBound, end: Option<FrameBound>) -> Self {
        self.frame = Some(WindowFrame { typ: FrameType::Rows, start, end });
        self
    }

    pub fn range(mut self, start: FrameBound, end: Option<FrameBound>) -> Self {
        self.frame = Some(WindowFrame { typ: FrameType::Range, start, end });
        self
    }

    pub fn groups(mut self, start: FrameBound, end: Option<FrameBound>) -> Self {
        self.frame = Some(WindowFrame { typ: FrameType::Groups, start, end });
        self
    }

    // Not in Node, Native curing method
    pub fn order_by<C, M>(mut self, cols: C) -> Self
        where
            C: IntoArg<Ordering<'a, DB, X>, M> {
        self.order_by = cols.into_args_erased();
        self
    }

    pub fn alias<A, T>(self, alias: A) -> Projection<'a, DB, X>
        where
            A: IntoIdentifier<T>,
            Self: IntoProjection<'a, DB, X> {
        self.into_projection().alias(alias)
    }
}

pub struct PartitionBy<'a, DB: Database, X> {
    operand: Operand<'a, DB>,
    _marker: PhantomData<X>,
}

impl<'a, DB: Database, X> PartitionBy<'a, DB, X> {
    pub fn new(operand: Operand<'a, DB>) -> Self {
        Self { operand, _marker: PhantomData }
    }
}

pub trait IntoPartitionBy<'a, DB: Database, X> {
    fn into_partition_by(self) -> PartitionBy<'a, DB, X>;
}

impl<'a, DB: Database, X> EraseX for PartitionBy<'a, DB, X> {
    type Erased = PartitionBy<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        PartitionBy { operand: self.operand, _marker: PhantomData }
    }
}
