// use crate::types::Postgres;
// use kova_core::dsl::StandardOperator;
// use kova_core::render::{PlatformRender, SelectContext};
// 
// impl<'ctc, 'a> PlatformRender<StandardOperator<'a, Postgres>, SelectContext<'ctc, 'a, Postgres>> for Postgres {
//     fn render(component: &StandardOperator<'a, Postgres>, ctx: &mut SelectContext<'ctc, 'a, Postgres>) {
//         match component {
//             StandardOperator::Eq(col, expr) => {
// 
//             },
// 
//             _ => { todo!() }
//         }
//     }
// }