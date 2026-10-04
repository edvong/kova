use crate::dsl::ExtFunction;
use crate::types::Postgres;
use kova_core::dsl::func::{Function, Window};
use kova_core::dsl::{Column, IntoOperand, Operand};
use std::marker::PhantomData;

// Unified user-facing wrapper leveraging the Type-State pattern.
// This prevents API noise and avoids complex macro_rules! while ensuring strict compile-time type boundaries.
pub struct Lag<T>(pub T);

impl<'a, T> Lag<T>
    where
        T: IntoLagState<'a> {
    pub fn offset(self, offset: i32) -> Lag<LagState<'a, T::X>> {
        let mut state = self.0.into_state();
        state.offset = Some(offset);
        // The outer layer remains `Lag`, but the internal generic `T` transitions to `LagState`.
        Lag(state)
    }

    pub fn default<V, K>(self, val: V) -> Lag<LagState<'a, T::X>>
        where
            V: IntoOperand<'a, T::X, Postgres, K, T::X> {
        // Strict type constraint: 'V' must flawlessly align with the originating column's type 'T::X'.
        // This completely eliminates runtime type mismatch errors when providing a fallback value.
        let mut state = self.0.into_state();
        state.default = Some(val.into_operand());
        Lag(state)
    }

    pub fn over<X>(self) -> Window<'a, Postgres, X> {
        let state = self.0.into_state();
        let func =
            ExtFunction::Lag(Box::new(state.base), state.offset, state.default.map(Box::new));

        // Escapes the internal type bounds ('T::X') to project a free generic 'X'.
        // This is critical for polymorphic tuple extraction during AST compilation (IntoArgs),
        // preventing E0277 trait resolution failures when chaining `.columns(...)`.
        Window::new(Function::from(func))
    }
}

// Internal compilation state.
// 'X' acts as the phantom anchor tying the raw AST operand back to its original Rust data type.
pub struct LagState<'a, X> {
    pub(crate) base: Operand<'a, Postgres>,
    pub(crate) offset: Option<i32>,
    pub(crate) default: Option<Operand<'a, Postgres>>,
    pub(crate) _marker: PhantomData<X>,
}

// State transition protocol.
// Allows seamless builder chaining from the initial `Column` down through intermediate `LagState` mutations.
pub trait IntoLagState<'a> {
    type X;
    fn into_state(self) -> LagState<'a, Self::X>;
}

// Initial state injection:
// Extracts the AST metadata (Identifier) from the heavy Column trait,
// securely trapping its target type 'U' inside the state's 'X' generic.
impl<'a, U> IntoLagState<'a> for Column<U, Postgres> {
    type X = U;
    fn into_state(self) -> LagState<'a, Self::X> {
        LagState {
            base: Operand::from_column(self.identifier()),
            offset: None,
            default: None,
            _marker: PhantomData,
        }
    }
}

// Accommodates by-reference column passing (e.g., `&Order::amount`).
impl<'a, U> IntoLagState<'a> for &Column<U, Postgres> {
    type X = U;
    fn into_state(self) -> LagState<'a, Self::X> {
        LagState {
            base: Operand::from_column(self.identifier()),
            offset: None,
            default: None,
            _marker: PhantomData,
        }
    }
}

// Identity transition:
// Allows chained builder methods (offset, default) to effortlessly pass the mutated state forward.
impl<'a, X> IntoLagState<'a> for LagState<'a, X> {
    type X = X;
    fn into_state(self) -> LagState<'a, X> {
        self
    }
}
