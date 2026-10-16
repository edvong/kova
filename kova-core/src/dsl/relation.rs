use crate::types::Relation;

pub trait IntoRelationList {
    fn relation_list() -> Vec<&'static str>;
}

pub trait SingleRelationMarker {}

impl<T: Relation> SingleRelationMarker for T {}

impl<T> IntoRelationList for T
where
    T: Relation + SingleRelationMarker,
{
    #[inline(always)]
    fn relation_list() -> Vec<&'static str> {
        vec![T::RELATION_NAME]
    }
}

macro_rules! impl_into_relation_list_tuple {
    ( $($T:ident),+ ) => {
        #[diagnostic::do_not_recommend]
        impl<$($T: Relation),+> IntoRelationList for ($($T,)+) {
            #[inline(always)]
            fn relation_list() -> Vec<&'static str> {
                vec![ $( $T::RELATION_NAME ),+ ]
            }
        }
    };
}

impl_into_relation_list_tuple!(T1);
impl_into_relation_list_tuple!(T1, T2);
impl_into_relation_list_tuple!(T1, T2, T3);
impl_into_relation_list_tuple!(T1, T2, T3, T4);
impl_into_relation_list_tuple!(T1, T2, T3, T4, T5);
impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6);
impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_into_relation_list_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
