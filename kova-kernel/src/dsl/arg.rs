use crate::dsl::EraseX;
use std::marker::PhantomData;

// 1. Direct passthrough (e.g. A -> A)
pub struct ArgDirect;

// 2. AST node conversion (Node<T>, Select, Condition, Table -> A)
pub struct ArgAst;

// 3. Blanket conversion for encodables (T: Encode<'a, DB, X> -> Bind<'a, DB, X>)
pub struct ArgEncode;

// Tuple flattening marker (handles 1..=32 elements)
pub struct ArgTuple<M>(PhantomData<M>);


// Guard: ensures only single-arg markers can go inside tuples to forbid nested tuples like ((a, b), c).
// Only implemented for the 3 ZST markers above, never for concrete AST or business types.
pub trait SingleArg {}
impl SingleArg for ArgDirect {}
impl SingleArg for ArgAst {}
impl SingleArg for ArgEncode {}

#[diagnostic::on_unimplemented(
    message = "cannot convert `{Self}` into SQL argument(s) of type `{A}`",
    label = "invalid SQL argument(s)",
    note = "expected a valid SQL construct or encodable value.\nif passing a tuple, ensure every element is a valid single argument (nested tuples are not allowed)."
)]
pub trait IntoArg<A, M>: Sized {
    const LEN: usize = 1;

    fn into_arg(self) -> A;

    #[inline(always)]
    fn into_args(self) -> Vec<A> {
        vec![self.into_arg()]
    }

    #[inline(always)]
    fn into_args_erased(self) -> Vec<A::Erased> where A: EraseX {
        vec![self.into_arg().erase_x()]
    }
}

// Identity passthrough: input is already target A
// (e.g. Projection with .alias() or Ordering with .asc())
impl<A> IntoArg<A, ArgDirect> for A {
    #[inline(always)]
    fn into_arg(self) -> A {
        self
    }
}

macro_rules! impl_into_arg_for_tuple {
    ( $( ($T:ident,$M:ident) ),+ ) => {
        #[diagnostic::do_not_recommend]
        impl<A, $($T,$M),+> IntoArg<A, ArgTuple<($($M,)+)>> for ($($T,)+)
            where
                $($M: SingleArg, $T: IntoArg<A,$M> ),+ {
            const LEN: usize = 0 $( + <$T as IntoArg<A,$M>>::LEN )+;

            #[inline(always)]
            fn into_arg(self) -> A {
                unreachable!("tuple cannot be converted into a single arg")
            }

            #[inline(always)]
            fn into_args(self) -> Vec<A> {
                #[allow(non_snake_case)]
                let ( $($T,)+ ) = self;
                vec![ $($T.into_arg() ),+ ]
            }

            #[inline(always)]
            fn into_args_erased(self) -> Vec<A::Erased> where A: EraseX {
                #[allow(non_snake_case)]
                let ( $($T,)+ ) = self;
                vec![ $($T.into_arg().erase_x() ),+ ]
            }
        }
    };
}

#[rustfmt::skip]
mod macros {
    use super::*;
    impl_into_arg_for_tuple!((T1, M1));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24), (T25, M25));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24), (T25, M25), (T26, M26));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24), (T25, M25), (T26, M26), (T27, M27));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24), (T25, M25), (T26, M26), (T27, M27), (T28, M28));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24), (T25, M25), (T26, M26), (T27, M27), (T28, M28), (T29, M29));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24), (T25, M25), (T26, M26), (T27, M27), (T28, M28), (T29, M29), (T30, M30));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24), (T25, M25), (T26, M26), (T27, M27), (T28, M28), (T29, M29), (T30, M30), (T31, M31));
    impl_into_arg_for_tuple!((T1, M1), (T2, M2), (T3, M3), (T4, M4), (T5, M5), (T6, M6), (T7, M7), (T8, M8), (T9, M9), (T10, M10), (T11, M11), (T12, M12), (T13, M13), (T14, M14), (T15, M15), (T16, M16), (T17, M17), (T18, M18), (T19, M19), (T20, M20), (T21, M21), (T22, M22), (T23, M23), (T24, M24), (T25, M25), (T26, M26), (T27, M27), (T28, M28), (T29, M29), (T30, M30), (T31, M31), (T32, M32));
}
