mod aggregate;
mod scalar;
mod window;
// mod can;
// mod macros;

pub use self::aggregate::{
    Aggregate, AggregateBuild, AggregateMarker, Avg, AvgMarker, Count, CountArg, CountMarker, Max,
    MaxMarker, Min, MinMarker, Sum, SumMarker, Wildcard,
};
pub use self::scalar::{
    Abs, AbsMarker, Length, LengthMarker, Lower, LowerMarker, Scalar, ScalarBuild, ScalarMarker,
    Upper, UpperMarker,
};
use crate::dsl::component::{Ext, Std};
pub use crate::dsl::function::window::{FrameBound, FrameType, Window};
use crate::dsl::{Component, Condition, Operand, Ordering};
use crate::types::{Database, Extension};

pub type ExtFunction<'a, DB> = <<DB as Database>::Extension<'a> as Extension<'a>>::Function<'a>;

pub struct Function<'a, DB: Database> {
    inner: Component<StandardFunction<'a, DB>, ExtFunction<'a, DB>>,
    quantifier: Option<Quantifier>,
    order_by: Vec<Ordering<'a, DB, ()>>,
    window: Option<Box<Window<'a, DB, ()>>>,
    filter: Option<Box<Condition<'a, DB, ()>>>,
}

impl<'a, DB: Database> Function<'a, DB> {
    pub fn from<T, M>(func: T) -> Self
    where
        T: IntoFunctionInner<'a, DB, M>, {
        func.into_function()
    }

    pub fn quantifier(mut self, quantifier: Quantifier) -> Self {
        self.quantifier = Some(quantifier);
        self
    }

    pub fn order_by(mut self, order_by: Vec<Ordering<'a, DB, ()>>) -> Self {
        self.order_by = order_by;
        self
    }

    pub fn window(mut self, window: Window<'a, DB, ()>) -> Self {
        self.window = Some(Box::new(window));
        self
    }

    pub fn filter(mut self, cond: Condition<'a, DB, ()>) -> Self {
        self.filter = Some(Box::new(cond));
        self
    }
}

pub trait SetDistinct {
    fn set_distinct(self, quantifier: Quantifier) -> Self;
}

pub trait SetWindow<'a, DB: Database, X> {
    fn set_window(self, window: Window<'a, DB, ()>) -> Self;
}

pub trait SetFilter<'a, DB: Database, X> {
    fn set_filter(self, cond: Condition<'a, DB, ()>) -> Self;
}

pub trait SetOrderBy<'a, DB: Database, X> {
    fn set_order_by(self, ordering: Vec<Ordering<'a, DB, ()>>) -> Self;
}

// impl<'a, DB: Database> SetDistinct for Function<'a, DB> {
//     fn set_distinct(mut self, quantifier: Quantifier) -> Self {
//         self.quantifier = Some(quantifier);
//         self
//     }
// }
//
// impl<'a, DB: Database, X> SetWindow<'a, DB, X> for Function<'a, DB> {
//     fn set_window(mut self, window: Window<'a, DB, ()>) -> Self {
//         self.window = Some(Box::new(window));
//         self
//     }
// }
//
// impl<'a, DB: Database, X> SetFilter<'a, DB, X> for Function<'a, DB> {
//     fn set_filter(mut self, cond: Condition<'a, DB, ()>) -> Self {
//         self.filter = Some(Box::new(cond));
//         self
//     }
// }

pub enum StandardFunction<'a, DB: Database> {
    Count(Box<Operand<'a, DB>>),
    Sum(Box<Operand<'a, DB>>),
    Max(Box<Operand<'a, DB>>),
    Min(Box<Operand<'a, DB>>),
    Avg(Box<Operand<'a, DB>>),

    Length(Box<Operand<'a, DB>>),
    Lower(Box<Operand<'a, DB>>),
    Upper(Box<Operand<'a, DB>>),
    Abs(Box<Operand<'a, DB>>),
}

#[derive(Clone, Copy)]
pub enum Quantifier {
    All,
    Distinct,
}

// pub trait IntoFunction<'a, DB: Database, X> {
//     fn into_function(self) -> Function<'a, DB>;
// }

pub trait IntoFunctionInner<'a, DB: Database, Marker> {
    fn into_function(self) -> Function<'a, DB>;
}

impl<'a, DB: Database> IntoFunctionInner<'a, DB, Std> for StandardFunction<'a, DB> {
    fn into_function(self) -> Function<'a, DB> {
        Function {
            inner: Component::standard(self),
            quantifier: None,
            order_by: Vec::new(),
            window: None,
            filter: None,
        }
    }
}

impl<'a, DB: Database> IntoFunctionInner<'a, DB, Ext> for ExtFunction<'a, DB> {
    fn into_function(self) -> Function<'a, DB> {
        Function {
            inner: Component::extension(self),
            quantifier: None,
            order_by: Vec::new(),
            window: None,
            filter: None,
        }
    }
}
