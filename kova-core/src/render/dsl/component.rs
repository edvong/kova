use crate::Result;
use crate::dsl::Component;
use crate::dsl::component::ComponentKind;
use crate::render::{Render, RenderContext, RenderDispatch};
use crate::types::Database;

impl<'a, 's, DB, S, E> Render<'a, 's, DB> for Component<S, E>
where
    DB: Database + RenderDispatch<'a, 's, E>,
    S: Render<'a, 's, DB>,
{
    #[inline(always)]
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        match &self.inner {
            ComponentKind::Standard(s) => s.render(ctx),
            ComponentKind::Extension(e) => DB::dispatch(e, ctx),
        }
    }
}
