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

pub trait IntoArgs<A>: Sized {
    fn size_hint(&self) -> usize;

    fn for_each_arg<F>(self, f: &mut F)
    where
        F: FnMut(A);

    #[inline(always)]
    fn into_args(self) -> Vec<A> {
        let mut vec = Vec::with_capacity(self.size_hint());
        self.for_each_arg(&mut |item| vec.push(item));
        vec
    }

    #[inline(always)]
    fn into_mapped_args<B, M>(self, mut map_fn: M) -> Vec<B>
    where
        M: FnMut(A) -> B,
    {
        let mut vec = Vec::with_capacity(self.size_hint());
        self.for_each_arg(&mut |item| vec.push(map_fn(item)));
        vec
    }
}

impl<T, A> IntoArgs<A> for T
where
    T: IntoArg<A> + SingleArgMarker,
{
    fn size_hint(&self) -> usize {
        1
    }

    #[inline(always)]
    fn for_each_arg<F>(self, f: &mut F)
    where
        F: FnMut(A)
    {
        f(self.into_arg());
    }
}


impl<A> IntoArgs<A> for Vec<A> {
    #[inline(always)]
    fn size_hint(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn for_each_arg<F>(self, f: &mut F)
    where
        F: FnMut(A)
    {
        for item in self {
            f(item);
        }
    }
}


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

            fn for_each_arg<F>(self, f: &mut F)
            where
                F: FnMut(A)
            {
                #[allow(non_snake_case)]
                let ( $($T,)+ ) = self;
                $(
                    $T.for_each_arg(f);
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