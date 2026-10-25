use crate::types::Postgres;
use kova_core::dsl::{Component, Conflict, Ext, IntoConflictTarget};

pub struct Constraint(pub &'static str);

impl<'a, X> IntoConflictTarget<'a, Postgres, Ext, X> for Constraint {
    #[inline(always)]
    fn into_conflict_target(self) -> Component<Vec<Conflict<'a, Postgres, ()>>, Self> {
        Component::extension(self)
    }
}
