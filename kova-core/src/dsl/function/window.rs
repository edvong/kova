use crate::dsl::function::{Function, StandardFunction};
use crate::dsl::{EraseX, IntoArg, IntoArgs, IntoIdentifier, IntoOperand, IntoOrdering, IntoProjection, OpAst, Operand, Ordering, Projection, SingleArgMarker};
use crate::types::Database;
use std::marker::PhantomData;

pub struct Window<'a, DB: Database, X> {
    pub(crate) func: Function<'a, DB>,
    pub(crate) partition_by: Vec<Operand<'a, DB>>,
    pub(crate) order_by: Vec<Ordering<'a, DB, ()>>,
    pub(crate) frame: Option<WindowFrame>,
    pub(crate) _marker: PhantomData<X>,
}

#[derive(Debug, Clone)]
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
    pub fn new(func: Function<'a, DB>) -> Self {
        Self {
            func,
            partition_by: Vec::new(),
            order_by: Vec::new(),
            frame: None,
            _marker: PhantomData,
        }
    }

    pub fn partition_by<C>(mut self, cols: C) -> Self
        where
            C: IntoArgs<Operand<'a, DB>> {
        self.partition_by = cols.into_args();
        self
    }

    pub fn order_by<C>(mut self, cols: C) -> Self
        where
            C: IntoArgs<Ordering<'a, DB, X>> {
        self.order_by = cols.into_mapped_args(|o| o.erase_x());
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

    pub fn alias<A: IntoIdentifier<T>, T>(self, alias: A) -> Projection<'a, DB, X> {
        self.into_projection().alias(alias)
    }

    #[inline(always)]
    fn into_func(self) -> Function<'a, DB> {
        Function::from(StandardFunction::Window(Box::new(self.erase_x())))
    }
}

impl<'a, DB: Database, X> EraseX for Window<'a, DB, X> {
    type Erased = Window<'a, DB, ()>;

    fn erase_x(self) -> Self::Erased {
        Window {
            func: self.func,
            partition_by: self.partition_by,
            order_by: self.order_by,
            frame: self.frame,
            _marker: PhantomData,
        }
    }
}

impl<'a, DB: Database, X> SingleArgMarker for Window<'a, DB, X> {}

impl<'a, DB: Database, X> IntoOperand<'a, (), DB, OpAst, X> for Window<'a, DB, X> {
    fn into_operand(self) -> Operand<'a, DB> {
        Operand::from_function(self.into_func())
    }
}

impl<'a, DB: Database, X> IntoOrdering<'a, DB, X> for Window<'a, DB, X> {
    #[inline(always)]
    fn into_ordering(self) -> Ordering<'a, DB, X> {
        Ordering::from_operand(self.into_operand())
    }
}

impl<'a, DB: Database, X> IntoProjection<'a, DB, X> for Window<'a, DB, X> {
    fn into_projection(self) -> Projection<'a, DB, X> {
        Projection::new(self.into_operand())
    }
}

impl<'a, DB: Database, X> IntoArg<Projection<'a, DB, X>> for Window<'a, DB, X> {
    #[inline(always)]
    fn into_arg(self) -> Projection<'a, DB, X> {
        self.into_projection()
    }
}
