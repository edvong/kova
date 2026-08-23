use crate::dsl::Component;
use crate::dsl::function::Function;
use crate::types::{ColumnMeta, Database, Extension};

pub type Projection<'a, DB> = Component<
    StandardProjection<'a, DB>,
    <<DB as Database>::Extension<'a> as Extension<'a>>::Projection,
>;

pub enum StandardProjection<'a, DB: Database> {
    Column(ColumnMeta, Option<ColumnMeta>),
    Function(Function<'a, DB>, Option<ColumnMeta>),
}

// Single Column into projection.
pub trait IntoProjection<'a, DB: Database> {
    fn into_projection(self) -> Projection<'a, DB>;
}

// Tuple into projection vec
pub trait Projectable<'a, DB: Database> {
    fn into_projections(self) -> Vec<Projection<'a, DB>>;
}

impl<'a, DB: Database> IntoProjection<'a, DB> for Projection<'a, DB> {
    fn into_projection(self) -> Projection<'a, DB> {
        self
    }
}

impl<'a, DB: Database> Projectable<'a, DB> for Projection<'a, DB> {
    fn into_projections(self) -> Vec<Projection<'a, DB>> {
        vec![self]
    }
}

// `Select::columns()` accept `Vec<Projection>`
impl<'a, DB: Database> Projectable<'a, DB> for Vec<Projection<'a, DB>> {
    fn into_projections(self) -> Vec<Projection<'a, DB>> {
        self
    }
}

macro_rules! impl_projectable_for_tuple {
    ( $($T:ident),+ ) => {
        impl<'a, DB: Database, $($T),+> Projectable<'a, DB> for ($($T,)+)
        where
            $( $T: IntoProjection<'a, DB> ),+
        {
            fn into_projections(self) -> Vec<Projection<'a, DB>> {
                #[allow(non_snake_case)]
                let ( $($T,)+ ) = self;
                vec![
                    $( $T.into_projection() ),+
                ]
            }
        }
    };
}

impl_projectable_for_tuple!(C1, C2);
impl_projectable_for_tuple!(C1, C2, C3);
impl_projectable_for_tuple!(C1, C2, C3, C4);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6, C7);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6, C7, C8);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6, C7, C8, C9);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6, C7, C8, C9, C10);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13);
impl_projectable_for_tuple!(C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24, C25
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24, C25, C26
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24, C25, C26, C27
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24, C25, C26, C27, C28
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24, C25, C26, C27, C28, C29
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24, C25, C26, C27, C28, C29, C30
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24, C25, C26, C27, C28, C29, C30, C31
);
impl_projectable_for_tuple!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21,
    C22, C23, C24, C25, C26, C27, C28, C29, C30, C31, C32
);
