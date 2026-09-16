// use crate::dsl::Component;
// use crate::render::Render;
//
// impl<C, S, E> Render<C> for Component<S, E>
// where
//     S: Render<C>,
//     E: Render<C>,
// {
//     fn render(&self, ctx: &mut C) {
//         match self {
//             Component::Standard(s) => s.render(ctx),
//             Component::Extension(e) => e.render(ctx),
//         }
//     }
// }
