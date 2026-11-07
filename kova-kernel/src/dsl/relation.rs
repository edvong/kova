use crate::types::Relation;

pub trait IntoRelationList {
    fn relation_list() -> Vec<&'static str>;
}

impl<T> IntoRelationList for T
    where
        T: Relation {
    #[inline(always)]
    fn relation_list() -> Vec<&'static str> {
        vec![T::NAME]
    }
}

macro_rules! impl_into_relation_list_tuple {
    ( $($T:ident),+ ) => {
        #[diagnostic::do_not_recommend]
        impl<$($T: Relation),+> IntoRelationList for ($($T,)+) {
            #[inline(always)]
            fn relation_list() -> Vec<&'static str> {
                vec![ $( $T::NAME ),+ ]
            }
        }
    };
}

#[rustfmt::skip]
mod macros {
    use super::*;
    impl_into_relation_list_tuple!(T1);
    impl_into_relation_list_tuple!(T1, T2);
    impl_into_relation_list_tuple!(T1, T2, T3);
    impl_into_relation_list_tuple!(T1, T2, T3, T4);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32);
}