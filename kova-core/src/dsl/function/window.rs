use crate::dsl::{EraseX, IntoArgs, IntoIdentifier, IntoProjection, Operand, Ordering, Projection};
use crate::types::Database;
use std::marker::PhantomData;

pub struct Window<'a, DB: Database, X> {
    pub(crate) partition_by: Vec<Operand<'a, DB>>,
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
    pub fn partition_by<C>(mut self, cols: C) -> Self
        where
            C: IntoArgs<Operand<'a, DB>> {
        self.partition_by = cols.into_args();
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
    pub fn order_by<C>(mut self, cols: C) -> Self
        where
            C: IntoArgs<Ordering<'a, DB, X>> {
        self.order_by = cols.into_mapped_args(|o| o.erase_x());
        self
    }

    pub fn alias<A, T>(self, alias: A) -> Projection<'a, DB, X>
        where
            A: IntoIdentifier<T>,
            Self: IntoProjection<'a, DB, X> {
        self.into_projection().alias(alias)
    }
}
