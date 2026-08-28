// use crate::dsl::StandardOperator;
// use crate::render::{PlatformRender, Render, RenderDispatch, SelectContext};
// use crate::types::Database;
//
// impl<'ctc, 'a, DB> Render<SelectContext<'ctc, 'a, DB>> for StandardOperator<'a, DB>
// where
//     DB: Database,
//     <DB as Database>::Extension<'a>: PlatformRender<Self, SelectContext<'ctc, 'a, DB>>,
// {
//     fn render(&self, ctx: &mut SelectContext<'ctc, 'a, DB>) {
//         // Explicitly specify the distributor, node type, and context type
//         <<DB as Database>::Extension<'a> as RenderDispatch<
//             StandardOperator<'a, DB>,
//             SelectContext<'ctc, 'a, DB>
//         >>::dispatch(self, ctx);
//     }
// }
