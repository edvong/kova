/// Marker trait: Declares that a type represents a "single argument".
/// This acts as a type-system firewall to prevent overlapping implementations
/// between individual primitive values and collections (like Tuples or Vecs).
pub trait SingleArgMarker {}

/// Conversion protocol for a single argument.
pub trait IntoArg<A> {
    fn into_arg(self) -> A;
}

// Reflexive implementation: The target parameter inherently satisfies its own conversion.
// This allows a value of type A to be directly used where an IntoArg<A> is expected.
impl<A> IntoArg<A> for A {
    fn into_arg(self) -> A {
        self
    }
}

/// Protocol for providing arguments in batch.
/// Designed for maximum performance: it intentionally avoids methods like `into_vec()`
/// to prevent unnecessary heap allocations, opting instead to append directly 
/// to an existing buffer.
pub trait IntoArgs<A> {
    /// Returns a hint of the number of arguments to be appended.
    /// Used to pre-allocate capacity in the target vector to avoid reallocation during pushes.
    fn size_hint(&self) -> usize;

    /// Appends the arguments directly into the underlying storage of the Bundle.
    fn append_args(self, out: &mut Vec<A>);
}

// ==========================================
// Blanket Impl: Automatic Batch Support for Single Args
// ==========================================
// Using the `IsSingleArg` marker trait, we can safely implement `IntoArgs` 
// for any single argument type without conflicting with Tuple or Vec implementations.
impl<T, A> IntoArgs<A> for T
where
    T: IntoArg<A> + SingleArgMarker,
{
    fn size_hint(&self) -> usize {
        1
    }

    fn append_args(self, out: &mut Vec<A>) {
        out.push(self.into_arg()); // Appends directly with zero allocation.
    }
}

// ==========================================
// Escape Hatch: Specialized Implementation for Vec
// ==========================================
// Allows users to pass dynamically sized slices or vectors of arguments.
impl<A> IntoArgs<A> for Vec<A> {
    fn size_hint(&self) -> usize {
        self.len()
    }

    fn append_args(mut self, out: &mut Vec<A>) {
        // Optimization: If the output buffer is empty, we perform a highly efficient 
        // O(1) pointer swap instead of copying the elements over.
        if out.is_empty() {
            std::mem::swap(out, &mut self);
        } else {
            out.extend(self); // Fallback to O(N) extension if the buffer already has data.
        }
    }
}

// ==========================================
// DSL Core: Macro-Expanded Implementations for Tuples
// ==========================================
// Rust lacks variadic generics. This macro unrolls tuple implementations at compile time,
// enabling a highly ergonomic DSL where users can pass arbitrary amounts of arguments 
// simply by grouping them in a tuple (e.g., `(arg1, arg2, arg3)`).
macro_rules! impl_into_args_for_tuple {
    ( $($T:ident),+ ) => {
        impl<A, $($T),+> IntoArgs<A> for ($($T,)+)
        where
            $( $T: IntoArgs<A> ),+
        {
            fn size_hint(&self) -> usize {
                #[allow(non_snake_case)]
                let ( $( $T, )+ ) = self;
                // Sums up the size hints of all elements within the tuple.
                0 $( + $T.size_hint() )+
            }

            fn append_args(self, out: &mut Vec<A>) {
                #[allow(non_snake_case)]
                let ( $($T,)+ ) = self;
                // Sequentially delegates the append operation to each tuple element.
                $(
                    $T.append_args(out);
                )+
            }
        }
    };
}

impl_into_args_for_tuple!(T1);
impl_into_args_for_tuple!(T1, T2);
impl_into_args_for_tuple!(T1, T2, T3);
impl_into_args_for_tuple!(T1, T2, T3, T4);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_into_args_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
);
impl_into_args_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32
);